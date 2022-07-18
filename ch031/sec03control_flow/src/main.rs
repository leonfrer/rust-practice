fn main() {
    // both of "if arms" be diffrent type
    // let condition = true;
    // let number = if condition { 1 } else { "two" };

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}
