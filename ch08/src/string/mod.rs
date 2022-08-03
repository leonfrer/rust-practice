pub fn update_string() {
    let mut s = "foo".to_string();
    // &let mut s = String::from("foo");
    s.push_str("ba");
    println!("{}", s);
    s.push('r');
    println!("{}", s);
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
