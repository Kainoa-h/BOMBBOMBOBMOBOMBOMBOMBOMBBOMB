impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let len = word.len() as i32;
        let mut pushes = 0;
        let full_blocks = len / 8;
        let remainder = len % 8;

        pushes += 8 * full_blocks * (full_blocks + 1) / 2;
        pushes += remainder * (full_blocks + 1);
        
        pushes
    }
}

struct  Solution{}

fn main(){
    assert_eq!(Solution::minimum_pushes("abcd".to_owned()), 4);
    assert_eq!(Solution::minimum_pushes("xycdefghij".to_owned()), 12);
}
