use std::io;

fn main() {
    println!("start a guess");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("error occurs");  //here expect means if error happens, console shows the str

    println!("your guess:{}",guess);
}
