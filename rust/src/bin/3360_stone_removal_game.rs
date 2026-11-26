impl Solution {
    pub fn can_alice_win(mut n: i32) -> bool {
        let mut remove = 10;
        while n >= remove {
            n -= remove;
            remove -= 1;
        }
        remove % 2 != 0
    }
}

struct Solution {}
