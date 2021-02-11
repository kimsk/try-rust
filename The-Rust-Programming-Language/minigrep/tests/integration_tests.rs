// to see println! use
// cargo test -- --nocapture


use minigrep::{Config, run};

#[test]
fn file_not_found_not_ok() {
    let query = String::from("query");
    let filename = String::from("file2.txt");
    let args = ["".to_string(), query, filename];
    let config = Config::new(&args).unwrap();

    let actual = run(config);
    assert!(actual.is_err());
    // match actual {
    //     Ok(()) => (),
    //     Err(_) => assert!(true),
    // }
}

#[test]
fn file_found_ok() {
    let query = String::from("query");
    let filename = String::from("./tests/file.txt");
    let args = ["".to_string(), query, filename];
    let config = Config::new(&args).unwrap();

    let actual = run(config);
    assert!(actual.is_ok());
}