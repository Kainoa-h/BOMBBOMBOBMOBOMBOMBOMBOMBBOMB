impl Solution {
    pub fn can_alice_win(mut n: i32) -> bool {
        let mut remove = 10;
        loop {
            n -= remove;
            remove -= 1;
            if n < 0 || remove < 0 {
                return remove % 2 == 0
            }
        }
    }
}

struct Solution {}
