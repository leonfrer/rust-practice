pub fn update_string() {
    let mut s = "foo".to_string();
    // &let mut s = String::from("foo");
    s.push_str("ba");
    println!("{}", s);
    s.push('r');
    println!("{}\n", s);
}

pub fn concatenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s = s1 + &s2;
    println!("the + operator use add method, which will takes ownership of self(the one before +)");
    println!("s: {}, s2: {}", s, s2);
    println!("You can also use format! macro to concat, which would not take ownership of any parameters");
    let s1 = String::from("Hello");
    let s2 = String::from("world");
    let s = format!("{}, {}", s1, s2);
    println!("s1: {}, s2: {}, s: {}\n", s1, s2, s);
}

pub fn internal() {
    let s = String::from("Hello");
    let len = s.len();
    println!("Hello String len: {}", len);
    let s = "你好";
    let len = s.len();
    println!("你好 &str len: {}", len);
    println!("你好 &str first byte: {}", s.as_bytes()[0]);
    println!("你好 &str first char: {}", &s[0..3]);
    for c in s.chars() {
        println!("char inter: {}", c);
    }
}
