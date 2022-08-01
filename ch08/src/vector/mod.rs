pub fn create_vector() {
    println!("create_vector");
    let v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);
    let v = vec![1, 2, 3];
    println!("v: {:?}\n", v);
}

pub fn modify_vector() {
    println!("modify_vector");
    println!("add elements:");
    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    println!("v: {:?}", v);
    println!("read element:");
    let first = &v[0];
    println!("The first element is {}", first);
    match v.get(2) {
        Some(value) => println!("The third element: {}", value),
        None => println!("There is no third element"),
    }
    v.push(8);
    println!("see comment!\n");
    // can't run this code, because you cannot have both immutable reference and mutable reference
    // println!("The first element is {}", first);
}
