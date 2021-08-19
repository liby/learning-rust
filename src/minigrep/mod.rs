use std::{env, error::Error, fs, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did't get a file name"),
        };

        let case_sensitive = match args.next() {
            Some(arg) => match arg.as_str() {
                "true" => true,
                "1" => true,
                _ => false,
            },
            // As long as the `CASE_INSENSITIVE` environment variable is set, no matter what the value is, it is considered case-insensitive.
            None => env::var("CASE_INSENSITIVE").is_err(),
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn config_all_args() {
        let args: vec::IntoIter<String> = vec![
            "target/debug/minigrep".to_string(),
            "frog".to_string(),
            "poem.txt".to_string(),
        ]
        .into_iter();

        let config = Config::new(args).unwrap();
        assert_eq!(config.query, "frog");
        assert_eq!(config.filename, "poem.txt");
    }

    #[test]
    fn config_insufficient_args() {
        let args: vec::IntoIter<String> = vec!["target/debug/minigrep".to_string()].into_iter();
        assert_eq!(Config::new(args).err(), Some("not enough arguments"));
    }
    #[test]
    fn file_does_not_exist() {
        let args: vec::IntoIter<String> = vec![
            "target/debug/minigrep".to_string(),
            "query".to_string(),
            "not_exist.txt".to_string(),
        ]
        .into_iter();

        assert!(run(Config::new(args).unwrap()).is_err());
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
