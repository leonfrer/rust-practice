use std::{collections::HashMap, hash::Hash};

pub struct Cacher<K, V, T>
where
    K: Eq + Hash,
    V: Copy,
    T: Fn(K) -> V,
{
    calculation: T,
    cache: HashMap<K, V>,
}

impl<K, V, T> Cacher<K, V, T>
where
    K: Eq + Hash + Copy,
    V: Copy,
    T: Fn(K) -> V,
{
    pub fn new(calculation: T) -> Cacher<K, V, T> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: K) -> V {
        match self.cache.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.cache.insert(arg, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v1 + 1, v2);

    let mut s = Cacher::new(|a: &str| a.len());
    let l1 = s.value("arg");
    assert_eq!(l1, 3);
    let l2 = s.value("Hello");
    assert_eq!(l2, 5);
}
