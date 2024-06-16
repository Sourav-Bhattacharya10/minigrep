use std::{fs, error::Error, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return  Err("Two CLI arguments required");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // Method1: passing true false in CLI args
        // let ignore_case_string = args[3].clone();

        // let ignore_case: bool = match ignore_case_string.parse() {
        //     Ok(v) => v || false,
        //     Err(_) => false,
        // };

        // Method2: setting env var in cli like: set IGNORE_CASE=true
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(val) => match val.parse() {
                Ok(v) => v || false,
                Err(inn_err) => {
                    eprintln!("parsing env variable IGNORE_CASE caused err : {:?}", inn_err);
                    false
                },
            },
            Err(err) => {
                eprintln!("reading env variable IGNORE_CASE caused err : {:?}", err);
                false
            },
        };

        Ok(Config { query, file_path,  ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }
    else {
        search(&config.query, &contents)
    };

    println!("\n");
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_new = query.to_lowercase();

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_new) {
            results.push(line);
        }
    }

    results
}