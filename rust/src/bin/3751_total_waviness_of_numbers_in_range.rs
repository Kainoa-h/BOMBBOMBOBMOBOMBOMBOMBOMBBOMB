struct Solution {}
impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        let mut wavi = 0;
        for num in num1..=num2 {
            let v = Self::to_vec(num);
            if v.len() < 3 {
                continue;
            }

            for i in 1..v.len() - 1 {
                if (v[i - 1] > v[i] && v[i + 1] > v[i]) || (v[i - 1] < v[i] && v[i + 1] < v[i]) {
                    wavi += 1;
                }
            }
        }
        wavi
    }

    fn to_vec(mut n: i32) -> Vec<u8> {
        let mut v = Vec::<u8>::new();
        while n != 0 {
            v.push((n % 10) as u8);
            n = n / 10;
        }
        v
    }
}
