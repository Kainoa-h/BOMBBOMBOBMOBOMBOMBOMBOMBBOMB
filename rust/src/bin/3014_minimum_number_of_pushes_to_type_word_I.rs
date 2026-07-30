impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut pushes = 0;
        for i in 0..word.len() as i32 {
            pushes += i/8 + 1;
        }

        pushes
    }
}

struct  Solution{}

fn main(){
    assert_eq!(Solution::minimum_pushes("abcd".to_owned()), 4);
    assert_eq!(Solution::minimum_pushes("xycdefghij".to_owned()), 12);
}
