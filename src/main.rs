use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("start a guess");

    let secret = rand::thread_rng().gen_range(1, 100);

    println!("generated number");
    println!("let's start the game");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            //The right way to suppress the warning is to actually write error handling in Chapter 9
            .expect("you are not guessing a right word");

        // parses a string to specific type of number you defined
        // reusing variables are important, called 'Shadow'
        let guess: u32 = match guess.trim().parse() {
            //here shows the error handling, two states are catched for results
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        // brackets are placeholders for variables
        print!("your guess:{}\t", guess);
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small\t"),
            Ordering::Greater => println!("Too big\t"),
            Ordering::Equal => {
                println!("fit!");
                //coorect and break the loop
                break;
            }
        }
    }
}
