use rand::Rng;
use std::cmp::Ordering;
use std::io;

// cargo doc --open
// generates doc!

fn main() {
    println!("Guess the #");

    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // returns io::Result
            .expect("Failed to read line"); // expect is required

        // convert a value of guess from string to u32
        // using Shadowing
        let guess: u32 = 
            match guess
                    .trim()
                    .parse() {
                Ok(num) => num,
                Err(_) => continue, // Use match expression to handling the error.
            };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
