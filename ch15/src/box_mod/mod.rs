#[derive(Debug, PartialEq)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;

use List::{Cons, Nil};

#[test]
fn test_box() {
    // recursive type
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:?}", list);

    // using Box<T> like a reference
    if let Cons(_, b) = list {
        assert_eq!(*b, Cons(2, Box::new(Nil)))
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

use std::ops::DerefMut;

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

#[test]
fn test_implicit_deref_coercions() {
    fn hello(name: &str) {
        println!("hello {}", name);
    }
    let mut n = MyBox::new(String::from("Rust"));
    // actually hello(&(*n)[..])
    // the [..] is String Slices
    hello(&n);

    *n = String::from("Cargo");
    assert_eq!(*n, String::from("Cargo"));
}
