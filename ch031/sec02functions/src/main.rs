fn main() {
    // statement and expression
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x: {}, y: {}", x, y);
    println!();

    // function with return values
    println!("function with return values");
    let x = five();
    println!("x: {}", x);
    println!();
}

fn five() -> u8 {
    5
}
