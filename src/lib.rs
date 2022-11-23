use std::error::Error;
use std::env;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    if config.case_insensitive {
        for line in search(&config.query, &contents) {
            println!("{}", line);
        }
    } else {
        for line in search_case_sensitive(&config.query, &contents) {
            println!("{}", line);
        }
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("more or fewer than two arguments") 
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_insensitive})
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut lines_matched = vec![];

    for line in content.lines() {
        if line.contains(query) {
            lines_matched.push(line);
        }
    }

    lines_matched
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines_matched = vec![];
    let query_lower = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lower) {
            lines_matched.push(line);
        }
    }
    
    lines_matched
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents),
        );
    }

    #[test]
    fn search_case_insensitive() {
        let query = "caps";
        let contents = "\
Hello,
line has no caps
line has CAPS
mixed CaPs
And this should not match.";

        assert_eq!(
            vec![
                "line has no caps",
                "line has CAPS",
                "mixed CaPs",
            ],
            search_case_sensitive(query, contents),
        );
    }
}
