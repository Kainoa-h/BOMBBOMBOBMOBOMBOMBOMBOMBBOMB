use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Entry {
    key: i32,
    value: i32,
    next: Option<usize>,
    prev: Option<usize>,
}

struct LRUCache {
    size: usize,
    len: usize,
    entry_vec: Vec<Entry>,
    map: HashMap<i32, usize>,
    head: Option<usize>, //lru
    tail: Option<usize>, //mru
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let n = capacity as usize;
        Self {
            size: n,
            len: 0,
            entry_vec: Vec::with_capacity(n),
            map: HashMap::with_capacity(n),
            head: None,
            tail: None,
        }
    }

    fn unstich_throw_back(&mut self, entry_idx: usize) {
        if self.tail == Some(entry_idx) {
            return;
        }
        let entry = self.entry_vec[entry_idx];
        if self.head == Some(entry_idx) {
            self.head = entry.next.or(self.tail);
        }
        if let Some(entry_before_idx) = entry.prev {
            self.entry_vec[entry_before_idx].next = entry.next;
        }
        if let Some(entry_after_idx) = entry.next {
            self.entry_vec[entry_after_idx].prev = entry.prev;
        }
        self.entry_vec[self.tail.unwrap()].next = Some(entry_idx);
        self.entry_vec[entry_idx].next = None;
        self.entry_vec[entry_idx].prev = self.tail;
        self.tail = Some(entry_idx);
    }

    fn get(&mut self, key: i32) -> i32 {
        let Some(&entry_idx) = self.map.get(&key) else {
            return -1;
        };
        if self.head == self.tail {
            return self.entry_vec[self.head.unwrap()].value;
        }
        self.unstich_throw_back(entry_idx);

        self.entry_vec[entry_idx].value
    }

    fn put(&mut self, key: i32, value: i32) {
        
        if let Some(&entry_idx) = self.map.get(&key) {
            self.entry_vec[entry_idx].value = value;
        } else if self.len < self.size {
            if let Some(tail) = self.tail {
                self.entry_vec[tail].next = Some(self.len);
            } else {
                self.head = Some(0);
            }
            self.entry_vec.push(Entry {
                key,
                value,
                next: None,
                prev: self.tail,
            });
            self.map.insert(key, self.len);
            self.tail = Some(self.len);
            self.len += 1;
            return;
        }

        if self.head == self.tail && self.map.contains_key(&key) {
            return;
        }

        if let Some(&entry_idx) = self.map.get(&key) {
            self.unstich_throw_back(entry_idx);
            return;
        }

        let old_entry_idx = self.head.unwrap();
        let old_entry = self.entry_vec[old_entry_idx];
        if let Some(entry_idx_after_old) = old_entry.next {
            self.entry_vec[entry_idx_after_old].prev = None;
        }

        self.map.remove(&old_entry.key);
        self.map.insert(key, old_entry_idx);
        self.head = old_entry.next.or(self.tail);
        self.entry_vec[old_entry_idx] = Entry {
            key,
            value,
            next: None,
            prev: self.tail,
        };
        if let Some(tail) = self.tail {
            self.entry_vec[tail].next = Some(old_entry_idx);
        }
        self.tail = Some(old_entry_idx);
    }
}

fn main() {
    // test1();
    test2();
    // test3();
}

fn test1() {
    let instruct = [
        "put", "put", "get", "put", "get", "put", "get", "get", "get",
    ];
    let params = [
        (1, 1),
        (2, 2),
        (1, -1),
        (3, 3),
        (2, -1),
        (4, 4),
        (1, -1),
        (3, -1),
        (4, -1),
    ];
    let expected = [0, 0, 1, 0, -1, 0, -1, 3, 4];
    let mut lru = LRUCache::new(2);

    for ((&ins, param), exp) in instruct.iter().zip(params).zip(expected) {
        if ins == "put" {
            lru.put(param.0, param.1);
            continue;
        }
        assert_eq!(exp, lru.get(param.0));
    }
}

fn test2() {
    let instruct = [ "put", "get", "put", "get", "get" ];
    let params = [
        (2, 2),
        (2, -1),
        (3, 3),
        (2, -1),
        (3, -1),
    ];
    let expected = [0, 2, 0, -1, 3];
    let mut lru = LRUCache::new(1);

    for ((&ins, param), exp) in instruct.iter().zip(params).zip(expected) {
        if ins == "put" {
            lru.put(param.0, param.1);
            continue;
        }
        assert_eq!(exp, lru.get(param.0));
    }
}


fn test3() {
    let instruct = ["put","put","get","put","put","get"];
    let params = [
        (2, 1),
        (2, 2),
        (2, -1),
        (1, 1),
        (4, 4),
        (2, -1),
    ];
    let expected = [0, 0, 2, 0, 0, -1];
    let mut lru = LRUCache::new(2);

    for ((&ins, param), exp) in instruct.iter().zip(params).zip(expected) {
        if ins == "put" {
            lru.put(param.0, param.1);
            continue;
        }
        assert_eq!(exp, lru.get(param.0));
    }
}
