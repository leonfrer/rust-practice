fn main() {
    let i1 = 9;
    let i2 = i1;
    println!("i1: {}, i2: {}", i1, i2);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1 is invalid, s2: {}", s2);

    let s1 = "Hello";
    let s2 = s1;
    println!("s1: {}, s2: {}", s1, s2);

    // tuple
    let t1 = (1, 0.2, "hello", '👌');
    let t2 = t1;
    println!("t1.3: {}, t2.3: {}", t1.3, t2.3);
    let t1 = (1, 0.2, "hello", '👌', String::from("Hello"));
    let t2 = t1;
    println!("t1 is invalid, t2.3: {}", t2.3);

    // array
    let a1 = [1, 2, 3, 4];
    let a2 = a1;
    println!("a1[1]: {}, a2[1]: {}", a1[1], a2[1]);
}
