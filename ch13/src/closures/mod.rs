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
        // if self.cache.contains_key(&arg) {
        //     self.cache[&arg]
        // } else {
        //     let v = (self.calculation)(arg);
        //     self.cache.insert(arg, v);
        //     v
        // }
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

// #[test]
// fn call_with_different_values() {
//     let mut c = Cacher::new(|a| a);
//     let v1 = c.value(1);
//     let v2 = c.value(2);
//     assert_eq!(v1 + 1, v2);
// }

struct City {
    name: String,
    population: i64,
    country: String,
}

fn sort_cities(cities: &mut Vec<City>, desc: bool) {
    cities.sort_by_key(|city| {
        if desc {
            -city.population
        } else {
            city.population
        }
    });
}
