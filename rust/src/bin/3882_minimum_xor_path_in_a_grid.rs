use std::ops::BitOrAssign;

const MAX_VALUE: usize = 1024;
const WORD_LEN: usize = 64;
const WORDS: usize = MAX_VALUE / WORD_LEN;

struct Mask([u64; WORDS]);

impl From<i32> for Mask {
    fn from(value: i32) -> Self {
        let value = value as usize;
        let mut new = Mask::new();
        new.0[value / WORD_LEN] = 1_u64 << (value % 64);
        new
    }
}

impl BitOrAssign<&Mask> for Mask {
    fn bitor_assign(&mut self, rhs: &Mask) {
        for (s, &x) in self.0.iter_mut().zip(rhs.0.iter()) {
            *s |= x;
        }
    }
}

impl Mask {
    fn new() -> Self {
        Mask([false; MAX_VALUE])
    }

    fn xor(&self, n: i32) -> Mask {
        let mut result = Mask::new();
        for (idx, _) in self.0.iter().enumerate().filter(|x| *x.1) {
            result.0[idx ^ n as usize] = true;
        }
        result
    }

    fn smallest(&self) -> i32 {
        match self.0.iter().position(|&x| x) {
            Some(x) => x as i32,
            None => unreachable!(),
        }
    }
}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let cols = grid[0].len();
        let mut dp = grid[0]
            .iter()
            .scan(0, |acc, &x| {
                *acc ^= x;
                Some(Mask::from(*acc))
            })
            .collect::<Vec<_>>();
        for row in grid.iter().skip(1) {
            for (idx, &cell) in row.iter().enumerate() {
                if idx != 0 {
                    let (left, top) = dp.split_at_mut(idx);
                    top[0] |= &left[idx - 1];
                }
                dp[idx] = dp[idx].xor(cell);
            }
        }

        dp[cols - 1].smallest()
    }
}

struct Solution {}
