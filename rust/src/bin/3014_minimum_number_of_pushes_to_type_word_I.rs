impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut len = word.len() as i32;
        let mut pushes = 0;
        let mut offset = 1;
        while len > 0 {
            pushes += if len < 9 { len * offset } else { 8 * offset };
            len -= 8;
            offset += 1;
        }

        pushes
    }
}

struct  Solution{}

fn main(){
    assert_eq!(Solution::minimum_pushes("abcd".to_owned()), 4);
    assert_eq!(Solution::minimum_pushes("xycdefghij".to_owned()), 12);
}
