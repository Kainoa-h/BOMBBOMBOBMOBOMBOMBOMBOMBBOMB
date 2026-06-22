impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut balloons = [0.0_f32; 5];
        for c in text.as_bytes() {
            match c {
                b'b' => balloons[0] += 1.0,
                b'a' => balloons[1] += 1.0,
                b'l' => balloons[2] += 0.5,
                b'o' => balloons[3] += 0.5,
                b'n' => balloons[4] += 1.0,
                _ => {}
            }
        }
        balloons.into_iter().map(|x| x as i32).min().unwrap()
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_owned()), 1);
}

