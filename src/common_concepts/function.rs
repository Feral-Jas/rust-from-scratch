pub fn expression() {
    println!("------------expression explained");
    let x = {
        let y = 5;
        y + 1
    };
    println!("{}", x)
}
