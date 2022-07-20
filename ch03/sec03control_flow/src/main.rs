fn main() {
    // both of "if arms" be& diffrent type
    // le&t condition = true;
    // let number = if condition { 1 } else { "two" };

    // Returning value with loop
    println!("using if in a let Statement");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    println!();

    // for loop
    println!("for loop");
    let a = [10, 11, 12, 13, 14];
    for num in a.iter() {
        println!("The value is: {}", num);
    }
    println!();

    println!("reverse of range");
    for num in (1..11).rev() {
        println!("{}!", num);
    }
}
