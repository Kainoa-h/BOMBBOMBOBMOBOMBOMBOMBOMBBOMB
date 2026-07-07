impl Solution {
    pub fn sum_and_multiply(mut n: i32) -> i64 {
        let mut arr = Vec::<i32>::new();
        while n != 0 {
            let digit = n % 10;
            if digit != 0 {
                arr.push(n % 10);
            }
            n /= 10;
        }

        let mut sum = 0;
        let mut non_zero = 0;
        let mut mul = 1;
        for num in arr {
            sum += num;
            non_zero += num * mul;
            mul *= 10;
        }

        sum as i64 * non_zero as i64
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::sum_and_multiply(10203004), 12340);
}
