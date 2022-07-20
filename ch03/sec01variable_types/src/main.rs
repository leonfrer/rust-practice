fn main() {
    // shadowing
    println!("shadowing");
    let x = 5;
    let x = x + 1;
    let mut x = x * 2;
    x += 1;
    println!("The value of x is: {}", x);
    println!();

    let i1: u8 = 57u8;
    println!("i1: {}", i1);

    // tuple destructuring
    println!("tuple destructuring");
    let tup: (f64, i32, u8) = (3.14, 500, 1);
    let (x, _y, _z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", _y);
    // tup still have ownership
    println!("The value of y is: {}", tup.1);
    println!();

    // array
    let a: [u8; 5] = [3, 3, 3, 3, 3];
    let x = a[0];
    println!("The value of x is: {}", x);
    let b = [10; 5];
    println!("{}", b[4]);
    // println!("{}", b);
    // let element = a[b[0]];
    // println!("The value of element is: {}", element);
    println!()
}
