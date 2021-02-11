use std::error::Error;
use std::fs;

// Box<dyn Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
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

}