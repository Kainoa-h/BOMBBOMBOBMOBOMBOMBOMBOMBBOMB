use rand::Rng;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            list: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let idx = self.list.len();
        match self.map.entry(val) {
            Occupied(_) => false,
            Vacant(x) => {
                x.insert(idx);
                self.list.push(val);
                true
            }
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.map.remove(&val) {
            Some(idx) => {
                self.list.swap_remove(idx);
                if let Some(moved_val) = self.list.get(idx) {
                    self.map.insert(*moved_val, idx);
                }
                true
            }
            None => false,
        }
    }

    fn get_random(&mut self) -> i32 {
        if self.list.len() == 1 {
            return *self.list.first().unwrap();
        }
        let mut rng = rand::thread_rng();
        let random_idx = rng.gen_range(0..self.list.len());

        self.list[random_idx]
    }
}

fn main() {
    let mut x = RandomizedSet::new();
    println!("insert 1: {:?}", x.insert(1));
    println!("list: {:?}", x.list);
    println!("remove 2: {:?}", x.remove(2));
    println!("list: {:?}", x.list);
    println!("insert 2: {:?}", x.insert(2));
    println!("list: {:?}", x.list);
    println!("list: {:?}", x.list);
    println!("get random: {:?}", x.get_random());
    println!("remove 1: {:?}", x.remove(1));
    println!("list: {:?}", x.list);
    println!("insert 2: {:?}", x.insert(2));
    println!("list: {:?}", x.list);
    println!("get random: {:?}", x.get_random());
}
