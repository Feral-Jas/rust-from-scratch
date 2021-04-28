pub fn expression() {
    println!("------------expression explained");
    //? Expressions do not include ending semicolons.
    //* If you add a semicolon to the end of an expression, you turn it into a statement
    //* which will then not return a value.

    let x = {
        let y = 5;
        y + 1
    };
    println!("expression result:{}", x)
}

pub fn fn_get_return() {
    println!("functions with return:{}", fn_with_return(4));
}

fn fn_with_return(input: i32) -> i32 {
    //* still no semicolon, need to get used to the null 'return' style
    let temp = 9;
    input + temp
}
