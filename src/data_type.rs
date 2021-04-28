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
    //? i/u 8-128
    //? isize/usize based on arch
    //* if not assigned type, i32 is default

    let x: u8 = 4;
    let y: i32 = 66;
    let z: u64 = 196;
    println!("x:{}\ty:{}\tz:{}", x, y, z);

    let q: isize = -5_432;
    let p: usize = 5_432;
    println!("dynamic value:{},{}", q, p);
}
pub fn floats() {
    //?  f32,f64 default(f64) with IEEE-754
    println!("------------float explained");

    let x = 3.2; //f64
    let y: f32 = 4.7; //f32
    println!("x:{}\ty:{}", x, y);
}
pub fn numeric_operation() {
    println!("------------numeric opoeration explained");

    let a = 3 + 2;
    let b = 5 - 3;
    let c = 7 * 12;
    let d = 9 / 3;
    let e = 46 % 13;
    println!("{},{},{},{},{}", a, b, c, d, e)
}
pub fn boolean() {
    println!("------------boolean explained");
    //? Boolean type are use as 'bool' in Rust
    //* still if you dont specify type, compiler will decide based on your value

    let flag = true;
    let flag1: bool = false;
    println!("flage={}", flag);
    println!("flag1={}", flag1);
}
pub fn character() {
    println!("------------character explained");
    //? Rust character type are defined by using ' ' single quotes
    //* Unicode, 4 bytes in size

    let emoji = '😈';
    let ascii = 'A';

    println!("emoji:{},ascii:{}", emoji, ascii);
}
pub fn tuple() {
    println!("------------tuple explained");
    //? Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    //? one tuple cannot be directly displayed, you should access into the element
    let tup1 = ('😈', 43, "james", true);
    let (a, b, c, d) = tup1;
    println!("{},{},{},{}", a, b, c, d);
    println!("{},{},{},{}", tup1.0, tup1.1, tup1.2, tup1.3)
}
pub fn array() {
    println!("------------array explained");
    //? Array's size is fixed rather than vector
    let weekdays = ["Mon", "Tue", "Wen", "Thu", "Fri", "Sat", "Sun"];
    let nums = [5; 8];
    let nums1: [i32; 3] = [4, 7, 1];
    println!(
        "{},{},{},{},{},{},{}",
        weekdays[0], weekdays[1], weekdays[2], weekdays[3], weekdays[4], weekdays[5], weekdays[6]
    );
    println!(
        "{},{},{},{},{}",
        nums[0], nums[1], nums[2], nums[3], nums[4]
    );
    println!("{},{},{}", nums1[0], nums1[1], nums1[2]);
}
