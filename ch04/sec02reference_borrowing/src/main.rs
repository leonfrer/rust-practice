fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("s1: {}, s1 len: {}", s1, len);

    let mut s1 = String::from("hello");
    // let r1 = &s1; // can't have other reference when mut reference exist
    let r2 = &mut s1;
    change(r2);
    println!("s1: {}", s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
} 
