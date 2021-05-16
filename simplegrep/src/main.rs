use clap::Clap;

#[derive(Clap, Debug)]
struct Opts {
    query: String,
    filename: String,
}

fn main() {
    let opts = Opts::parse();
    match simplegrep::simplegrep(&opts.query, &opts.filename) {
        Ok(file_match) => {
            for m in file_match {
                println!("== {}:{}", m.filename, m.line);
                println!("{}", m.context);
            }
        }
        Err(message) => {
            println!("Error: {}", message)
        }
    }
}
