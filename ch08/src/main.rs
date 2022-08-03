mod hash_map;
mod string;
mod vector;

fn main() {
    println!("Vector:");
    vector::create_vector();
    vector::modify_vector();
    vector::iterate_vector();
    vector::store_multiple_types();
    println!();

    println!("string:");
    string::update_string();
    string::concatenation();
    string::internal();
    println!();

    println!("HashMap:");
    hash_map::create_and_access_hash_map();
    hash_map::hash_map_ownership();
    hash_map::update_hash_map();
}
