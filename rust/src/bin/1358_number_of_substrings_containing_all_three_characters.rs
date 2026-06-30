use std::collections::LinkedList;
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut a_ll = LinkedList::<usize>::new();
        let mut b_ll = LinkedList::<usize>::new();
        let mut c_ll = LinkedList::<usize>::new();
        for (i, b) in s.bytes().enumerate() {
            match b {
                b'a' => a_ll.push_back(i),
                b'b' => b_ll.push_back(i),
                b'c' => c_ll.push_back(i),
                _ => {}
            }
        }

        let mut count = 0;

        for (i, b) in s.bytes().enumerate() {
            if a_ll.is_empty() || b_ll.is_empty() || c_ll.is_empty() {
                break;
            }
            let (a, b, c) = match b {
                b'a' => (
                    a_ll.pop_front().unwrap(),
                    *b_ll.front().unwrap(),
                    *c_ll.front().unwrap(),
                ),
                b'b' => (
                    *a_ll.front().unwrap(),
                    b_ll.pop_front().unwrap(),
                    *c_ll.front().unwrap(),
                ),
                b'c' => (
                    *a_ll.front().unwrap(),
                    *b_ll.front().unwrap(),
                    c_ll.pop_front().unwrap(),
                ),
                _ => panic!(),
            };

            let max = a.max(b).max(c);
            let slices = s.len() - max;
            count += slices as i32;
        }

        count
    }
}

struct Solution {}

fn main() {}
