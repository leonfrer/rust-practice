#[test]
fn test_spawn() {
    use std::thread;
    use std::time::Duration;
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawn thread count: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("main thread count: {i}");
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn test_move_closure_thread() {
    use std::thread;

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });
    handle.join().unwrap();
}
