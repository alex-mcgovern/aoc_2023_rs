use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut sum: u64 = 0;

    for line in contents.lines() {
        let digits: String = line.chars().filter(|c| c.is_digit(10)).collect();

        let first: char = digits.chars().next().unwrap();
        let last: char = digits.chars().last().unwrap();

        let concatenated = format!("{}{}", first, last);
        let parsed: u64 = u64::from_str_radix(&concatenated, 10).unwrap();

        sum += parsed;

        dbg!(digits);
        dbg!(first);
        dbg!(last);
        dbg!(parsed);
        dbg!(sum);
    }

    dbg!(sum);

    Ok(())
}

/**
 * Slightly overbuilt solution for building a config struct from the command line arguments.
 * Based on examples from https://doc.rust-lang.org/book/
 */
struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}
