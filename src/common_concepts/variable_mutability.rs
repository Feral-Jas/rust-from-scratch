pub fn mutable() {
    println!("------------mutable and immutable explained");

    //* In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change
    //? which to choose?
    //* large structure is faster using mutable variables
    //* small structure is more readable using immutable variables

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

pub fn constant() {
    println!("------------constant explained");

    //? Uppercase and type annotation is a must
    //* 1. cannot use 'mut' with 'const'
    //* 2. const can be decalred at any scope including global scope
    //* 3. const can only be set with 'const' expression rather than function returns
    //* underscore can be used for readable codes in rust
    const MAX_POINTS: i32 = 7_000;
    println!("MAX_POINTS is {}", MAX_POINTS);
}

pub fn shadow() {
    println!("------------shadowing explained");

    //? we are using 'let' to reassign value to variable
    //* still immutable after transformation
    //* can cross types, which 'mut' keyword cannot
    let x = "helloworld";
    let x = x.len();
    let x = x + 1;
    let x = x * 3;
    println!("shadow result: x = {}", x);
}
