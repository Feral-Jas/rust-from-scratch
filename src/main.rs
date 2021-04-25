use std::io;

fn main() {
    println!("start a guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("you are not guessing a right word");
    //The right way to suppress the warning is to actually write error handling in Chapter 9

    println!("your guess:{}", guess);
    //brackets are placeholders for variables
}
