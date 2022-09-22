#[test]
fn test_single_value_with_single_producer() {
    use std::sync::mpsc;
    use std::thread;
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got {received}");
}

#[test]
fn test_multiple_value_with_single_producer() {
    use std::sync::mpsc;
    use std::thread;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("Hello").unwrap();
        tx.send("Rust").unwrap();
        tx.send("Cargo").unwrap();
    });
    for received in rx {
        println!("Got :{received}");
    }
}

#[test]
fn test_multiple_value_with_multiple_producer() {
    use std::sync::mpsc;
    use std::thread;
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx1.send("Hello").unwrap();
        tx1.send("Rust").unwrap();
        tx1.send("Cargo").unwrap();
    });

    thread::spawn(move || {
        tx.send("more").unwrap();
        tx.send("value").unwrap();
        tx.send("for").unwrap();
        tx.send("you").unwrap();
    });
    for received in rx {
        println!("Got :{received}");
    }
}
