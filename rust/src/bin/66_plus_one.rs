impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = false;
        for d in digits.iter_mut().rev() {
            if *d < 9 {
                *d += 1;
                carry = false;
                break;
            } else {
                *d = 0;
                carry = true;
            }
        }
        if carry {
            let mut d = Vec::with_capacity(digits.len() + 1);
            d.push(1);
            d.append(&mut digits);
            digits = d;
        }
        digits
    }
}

struct Solution {}
