use std::collections::HashMap;

pub fn create_and_access_hash_map() {
    println!("two ways to create HashMap");
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 20);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 20];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // let scores: HashMap<String, i32> = teams.iter().zip(initial_scores.iter()).collect(); // this one doesn't work

    // access
    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        None => println!("there is no {} team", team_name),
        Some(value) => println!("{} team initial score: {}", team_name, value),
    }
    let team_name = String::from("Red");
    match scores.get(&team_name) {
        None => println!("there is no {} team", team_name),
        Some(value) => println!("{} team initial score: {}", team_name, value),
    }
    for ele in &scores {
        println!("element of HashMap: {:?}", ele);
    }
    for (k, v) in &scores {
        println!("key: {}, v: {}", k, v);
    }
}

pub fn hash_map_ownership() {
    let i1 = 1;
    let s1 = String::from("one");
    let mut map = HashMap::new();
    map.insert(i1, s1);
    println!("types that inplement Copy traits, the values are copied into the hash map");
    println!("i1: {} still avaliable, unlike s1", i1);
    println!(
        "if insert reference to value into hash map,
the value still avliable, 
but the values that the references point to must valid 
for at least as long as the hash map is valid\n"
    )
}

pub fn update_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("scores: {:?}", scores);
    scores.insert(String::from("Blue"), 20);
    println!("overwrited scores: {:?}", scores);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("after entry insert: {:?}", scores);

    let text = "hello world hello hello world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("update a value based on the old value {:?}", map);
}
