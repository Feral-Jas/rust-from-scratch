fn main() {
    mutable();
    constant();
    shadow();
}
fn mutable() {
    println!("------------mutable and immutable variable explained");

    //* In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change
    //? which to choose?
    //* large structure is faster using mutable variables
    //* small structure is more readable using immutable variables

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
fn constant() {
    println!("------------constant variable explained");

    //? Uppercase and type annotation is a must
    //* 1. cannot use 'mut' with 'const'
    //* 2. const can be decalred at any scope including global scope
    //* 3. const can only be set with 'const' expression
    const Y: i32 = 7;
    println!("{}", Y);
}
fn shadow() {}
