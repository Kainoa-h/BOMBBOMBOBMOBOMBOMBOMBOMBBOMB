impl Solution {
    pub fn max_sum_of_squares(mut num: i32, mut sum: i32) -> String {
        let mut result = String::new();
        while num > 0 && sum > 0 {
            if sum > 9 {
                result.push('9');
                sum -= 9;
            } else {
                result.push_str(&sum.to_string());
                sum = 0;
            }

            num -= 1;
        }

        if sum > 0 {
            return String::new();
        }

        for _ in 0..num {
            result.push('0');
            
        }
        
        result
    }
}

struct Solution {}
