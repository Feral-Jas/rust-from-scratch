//? Rust is a 'statically typed language'
//? which means that it must know the types of all variables at compile time

//? Scalar Types
//? Rust has four primary scalar types:
//* integers
//* floating-point numbers
//* Booleans
//* characters

pub fn integers() {
    println!("------------integer explained");
    let x: u8 = 4;
    let y: i32 = 66;
    let z: u64 = 196;
    println!("x:{}\ty:{}\tz:{}", x, y, z);
    //? i/u 8-128
    //? isize/usize based on arch
}
pub fn floats() {
    //?  f32,f64 default(f64) with IEEE-754
    println!("------------float explained");
    let x = 3.2; //f64
    let y: f32 = 4.7; //f32
    println!("x:{}\ty:{}", x, y);
}
