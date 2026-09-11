impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let n = digits.len();
        let mut seen = [false; 1000];
        let mut count = 0;
        for ia in 0..n {
            if digits[ia] == 0 { continue; }

            for ib in 0..n {
                if ib == ia { continue; }

                for ic in 0..n {
                    if ic == ia || ic == ib || digits[ic] % 2 != 0 { continue; }

                    let num = (digits[ia] * 100 + digits[ib] * 10 + digits[ic]) as usize;
                    count += i32::from(!seen[num]);
                    seen[num] = true;
                }
            }
        }
        count
    }
}

struct Solution {}
