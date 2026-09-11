impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let bucket = digits.into_iter().fold([0; 10], |mut acc, x| {
            acc[x as usize] += 1;
            acc
        });

        let mut count = 0;
        for i in (100..1000).step_by(2) {
            let mut n = i;
            let mut b = [0;10];
            for _ in 0..3 {
                b[(n % 10) as usize] += 1;
                n /= 10;
            }
            count += i32::from(b.iter().zip(&bucket).all(|(c, t)| c <= t));
        }
        count
    }
}

struct Solution {}
