use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("start a guess");

    let secret = rand::thread_rng().gen_range(1, 100);

    println!("generated number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("you are not guessing a right word");
    //The right way to suppress the warning is to actually write error handling in Chapter 9

    let guess: u32 = guess.trim().parse().expect("plz type a number");
    println!("your guess:{}", guess);
    //brackets are placeholders for variables
    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("fit!"),
        Ordering::Greater => println!("Too big"),
    }
}
