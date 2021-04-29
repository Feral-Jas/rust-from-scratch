pub fn if_else() {
    println!("------------if else explained");
    //? if statement accept a boolean variable
    //* Rust will not automatically try to convert non-Boolean types to a Boolean

    let mut count = 0;
    loop {
        if count < 3 {
            println!("{}<3", count);
            count = count + 1;
        } else if count < 9 {
            println!("3=<{}<9", count);
            count = count + 1;
        } else {
            break;
        }
    }
}

pub fn if_in_let() {
    println!("------------if in let explained");
    //* if&else in let should return the same type

    let condition = true;
    let result = if condition { 2 } else { 4 };
    println!("result from if in let:{}", result)
}

pub fn loop_fn() {
    println!("------------loop explained");
    //? loop will execute forever
    //* use 'break' and variable to jump out and return the value

    let mut count = 0;
    let result = loop {
        count = count + 1;
        if count > 10 {
            break count;
        }
    };
    println!("result from loop:{}", result);
}

pub fn while_fn() {
    println!("------------while explained");
    //* while function

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("element is {}", arr[index]);
        index = index + 1;
    }
}

pub fn for_fn() {
    println!("------------for explained");
    //? for function

    let a = [10, 20, 30, 40, 50];
    for item in a.iter() {
        println!("element is {}", item);
    }
}
