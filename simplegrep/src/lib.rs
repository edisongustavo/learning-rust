use std::borrow::Borrow;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(PartialEq)]
pub struct FileMatch {
    pub filename: String,
    pub context: String,
    pub line: usize,
}

pub fn find_matches<LinesT>(lines_it: LinesT, query: &str) -> Vec<(String, usize)>
where
    LinesT: Iterator,
    LinesT::Item: Borrow<str>,
{
    let mut all_matches = Vec::new();
    for (line_number, line) in lines_it.enumerate() {
        let line_str = line.borrow(); //TODO: I don't fully understand this right now.
        let matches: Vec<_> = line_str.match_indices(query).collect();
        if !matches.is_empty() {
            let mut context = format!("{}\n", line_str);
            let mut indicator = " ".repeat(line_str.len());
            let slice = unsafe { indicator.as_bytes_mut() };

            for (index, match_result) in matches {
                slice[index..(index + match_result.len())].fill(b'^');
            }
            context.push_str(indicator.as_str());
            all_matches.push((context, line_number + 1));
        }
    }
    return all_matches;
}

pub fn simplegrep(query: &str, filename: &str) -> Result<Vec<FileMatch>, String> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let lines = io::BufReader::new(file).lines();
    let lines = lines.map(|item| item.unwrap());
    let matches = find_matches(lines, query);
    let ret = matches
        .iter()
        .map(|(context, line_number)| FileMatch {
            filename: String::from(filename),
            context: context.clone(), // TODO: How to avoid this clone() ?
            line: *line_number,
        })
        .collect();
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[rstest]
    fn test_simplegrep_file_does_not_exist() {
        let result = simplegrep("my-query", "inexistent-file");
        assert_eq!(
            result.err(),
            Some(String::from("No such file or directory (os error 2)"))
        );
    }

    #[rstest]
    fn test_simplegrep_file_empty() {
        let file = NamedTempFile::new().unwrap();
        let result = simplegrep("my-query", file.path().to_str().unwrap());
        assert!(result.unwrap().is_empty());
    }

    #[rstest]
    fn test_simplegrep_file_with_matching_contents() {
        let mut file = NamedTempFile::new().unwrap();
        let contents = indoc! {r#"
            foo bar
            bar
        "#};
        file.write_all(contents.as_bytes()).unwrap();
        let filename = file.path().to_str().unwrap();
        let expected_context = indoc! {r#"
            foo bar
            ^^^    "#};
        let actual = &simplegrep("foo", filename).unwrap()[0];
        assert_eq!(actual.filename, filename);
        assert_eq!(actual.context, expected_context);
    }

    #[rstest]
    fn test_find_matches() {
        let contents: &str = indoc! {r#"
            foo
            foo bar foo
            foo foo
            foofoo
            bar
        "#};
        let res = find_matches(contents.trim().split("\n"), "foo");
        let actual: Vec<&str> = res
            .iter()
            .map(|(context, _line_number)| context.as_str())
            .collect();
        for s in &actual {
            println!("{}", s);
        }
        let expected: Vec<&str> = vec![
            indoc! {r#"
                foo
                ^^^
            "#},
            indoc! {r#"
                foo bar foo
                ^^^     ^^^
            "#},
            indoc! {r#"
                foo foo
                ^^^ ^^^
            "#},
            indoc! {r#"
                foofoo
                ^^^^^^
            "#},
        ]
        .iter()
        .map(|s| s.trim())
        .collect();
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_find_matches2() {
        let contents: &str = indoc! {r#"
            [package]
        "#};
        let res = find_matches(contents.trim().split("\n"), "a");
        let actual: Vec<&str> = res
            .iter()
            .map(|(context, _line_number)| context.as_str())
            .collect();
        for s in &actual {
            println!("{}", s);
        }
        let expected: Vec<&str> = vec![indoc! {r#"
            [package]
              ^  ^   "#}];
        // .iter()
        // .map(|s| s)
        // .collect();
        assert_eq!(actual, expected);
    }
}
