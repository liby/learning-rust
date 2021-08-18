use std::{env, error::Error, fs, process};

fn grep() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        if args.len() > 3 {
            case_sensitive = match args[3].as_str() {
                "true" => true,
                "1" => true,
                _ => false,
            }
        }

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
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_all_args() {
        let args = [
            "target/debug/minigrep".to_string(),
            "frog".to_string(),
            "poem.txt".to_string(),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, args[1]);
        assert_eq!(config.filename, args[2]);
    }

    #[test]
    fn config_insufficient_args() {
        let args = ["target/debug/minigrep".to_string()];
        assert_eq!(Config::new(&args).err(), Some("not enough arguments"));
    }
    #[test]
    fn file_does_not_exist() {
        let args = [
            "target/debug/minigrep".to_string(),
            "query".to_string(),
            "not_exist.txt".to_string(),
        ];

        assert!(
            run(Config::new(&args).unwrap()).is_err(),
            "The test parameters are `{:#?}`",
            args
        );
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
