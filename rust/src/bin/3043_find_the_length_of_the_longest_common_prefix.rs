struct Solution {}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut trie = Trie::default();
        let (to_insert, to_search) = if arr1.len() > arr2.len() {
            (&arr2, &arr1)
        } else {
            (&arr1, &arr2)
        };
        for &x in to_insert {
            trie.insert(x);
        }
        let mut largest = 0;
        for &x in to_search {
            largest = largest.max(trie.count(x));
        }

        largest
    }
}

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 10],
}

#[derive(Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn insert(&mut self, num: i32) {
        let mut node = &mut self.root;
        let (arr, start_idx) = Self::get_arr(num);
        for &n in &arr[start_idx..9] {
            node = node.children[n].get_or_insert_default()
        }
    }

    fn count(&self, num: i32) -> i32 {
        let mut count = 0;
        let mut node = &self.root;
        let (arr, start_idx) = Self::get_arr(num);
        for &n in &arr[start_idx..9] {
            let Some(x) = &node.children[n] else {
                break;
            };
            node = x;
            count += 1;
        }
        count
    }
    fn get_arr(mut num: i32) -> ([usize; 9], usize) {
        let mut arr = [0; 9];
        let mut start = 9;
        while num != 0 {
            start -= 1;
            arr[start] = (num % 10) as usize;
            num /= 10;
        }
        (arr, start)
    }
}

fn main() {
    assert_eq!(
        Solution::longest_common_prefix(vec![1, 10, 100], vec![1000]),
        3
    );

    assert_eq!(
        Solution::longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4]),
        0
    );

    assert_eq!(
        Solution::longest_common_prefix(vec![1, 1, 1], vec![1, 1, 1]),
        1
    );
}
