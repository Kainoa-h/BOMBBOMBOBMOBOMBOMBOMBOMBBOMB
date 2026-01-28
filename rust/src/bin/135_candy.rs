struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() == 1 {
            return 1;
        }

        let mut candies: Vec<i32> = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }
        for i in (0..ratings.len()).rev().skip(1) {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}

fn what(ratings: Vec<i32>, ans: i32) {
    println!("{:?}", ratings);
    assert_eq!(Solution::candy(ratings), ans);
}

fn main() {
    what(vec![1, 0, 2], 5);
    what(vec![1, 2, 2], 4);
    what(vec![1], 1);
    what(vec![1, 2, 87, 87, 87, 2, 1], 13);
}
