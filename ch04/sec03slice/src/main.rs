fn main() {
    let s = String::from("Hello World");
    let word = first_word(&s);
    println!("first word: {}", word);
    let word = first_word(&s[..]);
    println!("first word: {}", word);
    // s.clear(); // mutable borrow occurs here

    let s1 = "Hello World";
    let word = first_word(&s1);
    println!("first word: {}", word);
    let word = first_word(&s1[..]);
    println!("first word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
