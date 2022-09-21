use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

#[test]
fn test_rc_share_data() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("count after a = {}", Rc::strong_count(&a));
    {
        let b = Cons(10, Rc::clone(&a));
        println!("count after b = {}", Rc::strong_count(&a));
    }
    println!("count after b goes out of scope = {}", Rc::strong_count(&a));
    let c = Cons(20, Rc::clone(&a));
    println!("count after c = {}", Rc::strong_count(&a));
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger: messenger,
            value: 0,
            max: max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        let mut msg = Option::None;
        if percentage_of_max >= 1.0 {
            msg = Option::Some("self.messenger.send(msg)");
        } else if percentage_of_max >= 0.9 {
            msg = Option::Some("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.7 {
            msg = Option::Some("Warning: You've used up over 70% of your quota!");
        }
        if let Some(m) = msg {
            self.messenger.send(m);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messagers: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messagers: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messagers.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn test_ref_cell() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messagers.borrow().len(), 1);
    }
}
