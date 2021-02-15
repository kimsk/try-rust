#![allow(dead_code, unused_variables, unused_assignments)]
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    //let config = parse_config(&args);
    // let config = Config::new0(&args);

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // let contents: String = fs::read_to_string(config.filename)
    //         .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

    // We use if let rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1) if it does.
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
