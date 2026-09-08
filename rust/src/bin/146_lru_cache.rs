use std::collections::HashMap;

#[derive(Clone, Copy)]
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

    fn get(&mut self, key: i32) -> i32 {
        let Some(&entry_idx) = self.map.get(&key) else {
            return -1;
        };
        let entry = self.entry_vec[entry_idx];
        if self.head == Some(entry_idx) {
            self.head = entry.next;
        }
        if let Some(entry_before_idx) = entry.prev {
            self.entry_vec[entry_before_idx].next = entry.next;
        }
        if let Some(entry_after_idx) = entry.next {
            self.entry_vec[entry_after_idx].prev = entry.prev;
        }
        self.entry_vec[self.tail.unwrap()].next = Some(entry_idx);
        self.entry_vec[entry_idx].next = None;
        self.tail = Some(entry_idx);

        self.entry_vec[entry_idx].value
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.len < self.size {
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
        let old_entry_idx = self.head.unwrap();
        let old_entry = self.entry_vec[old_entry_idx];
        self.map.remove(&old_entry.key);
        self.head = old_entry.next;
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
