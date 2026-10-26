impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        match digits.iter().rposition(|&d| d < 9) {
            Some(i) => {
                digits[i] += 1;
                digits[i + 1..].fill(0);
            }
            None => {
                digits.fill(0);
                digits.insert(0, 1);
            }
        };
        digits
    }
}

struct Solution {}
