use std::error::Error;
use std::fs;

// Box<dyn Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

// Notice that we need an explicit lifetime 'a defined in the signature of search and used with the contents argument and the return value.

// In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).

// In other words, we tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument. This is important! The data referenced by a slice needs to be valid for the reference to be valid;
// Other programming languages don’t require you to connect arguments to return values in the signature.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results 
}

// https://stackoverflow.com/questions/57234140/how-to-assert-errors-in-rust
// Result<T, E> only implements PartialEq when T and E also implement PartialEq
#[derive(Debug, PartialEq)]
pub struct Config {
    query: String,
    filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_enough_arguments() {
        let args = ["minigrep".to_string(), "filename".to_string()];
        let actual = Config::new(&args);
        let expected = Err("not enough arguments");
        assert_eq!(expected, actual);
    }

    #[test]
    fn enough_arguments() {
        let query = "query";
        let filename = "filename";
        let args = ["minigrep".to_string(), String::from(query), String::from(filename)];
        let actual = Config::new(&args);
        let expected: Result<Config, &'static str> = Ok(Config { query: query.to_string(), filename: filename.to_string()});
        assert_eq!(expected, actual);
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

}