use velcro::hash_set;

/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!
 */
pub fn convert_words(s: &str) -> String {
    let words = s.split_ascii_whitespace();
    let mut ret = String::with_capacity(s.len());
    for word in words {
        let vowels = hash_set!['a', 'e', 'i', 'o', 'u'];
        let first_letter = word.chars().nth(0).unwrap();
        if vowels.contains(&first_letter) {
            let formatted_word = format!("{}-hay", word);
            ret += &formatted_word;
        } else {
            let word = String::from(word);
            let (_, word_without_first_letter) = word.split_at(1);
            let formatted_word = format!("{}-{}ay", word_without_first_letter, first_letter);
            ret += &formatted_word;
        }
        ret += " ";
    }
    return String::from(ret.trim());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_words() {
        assert_eq!(convert_words("Foo bar is fun"), "oo-Fay ar-bay is-hay un-fay");
    }
}