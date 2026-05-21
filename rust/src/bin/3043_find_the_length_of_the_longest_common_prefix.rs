struct Solution {}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        let mut arrs = [arr1, arr2];
        arrs.sort_unstable_by_key(|a| a.len());
        for &x in &arrs[0] {
            trie.insert(x);
        }
        let mut largest = 0;
        for &x in &arrs[1] {
            largest = largest.max(trie.count(x));
        }

        largest
    }
}

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 10],
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, num: i32) {
        let mut node = &mut self.root;
        let (arr, start_idx) = Self::get_arr(num);
        for &n in &arr[start_idx..9] {
            node = node.children[n as usize].get_or_insert_default()
        }
    }

    fn count(&self, num: i32) -> i32 {
        let mut count = 0;
        let mut node = &self.root;
        let (arr, start_idx) = Self::get_arr(num);
        for &n in &arr[start_idx..9] {
            if let Some(x) = &node.children[n as usize] {
                node = x;
                count += 1;
            } else {
                break;
            }
        }
        count
    }
    fn get_arr(mut num: i32) -> ([i32; 9], usize) {
        let len = num.ilog10() + 1;
        let mut arr = [0; 9];
        let mut idx = 0;
        while num != 0 {
            arr[idx] = num % 10;
            idx += 1;
            num /= 10;
        }
        arr.reverse();
        (arr, 9 - len as usize)
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
