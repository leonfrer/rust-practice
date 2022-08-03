mod hash_map;
mod string;
mod vector;

fn main() {
    println!("Vector:");
    vector::create_vector();
    vector::modify_vector();
    vector::iterate_vector();
    println!();

    println!("string:");
    string::update_string();
    string::internal();
    println!();
}
