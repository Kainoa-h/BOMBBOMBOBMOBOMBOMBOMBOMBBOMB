struct Solution {}
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut result = String::new();
        let roman = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let nums = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        for i in 0..roman.len() {
            while num >= nums[i] {
                num -= nums[i];
                result.push_str(roman[i]);
            }
        }

        result
    }
}

fn what(num: i32, ans: String) {
    println!("{}", num);
    assert_eq!(Solution::int_to_roman(num), ans);
}

fn main() {
    what(3, "III".to_owned());
    what(10, "X".to_owned());
    what(4, "IV".to_owned());
    what(58, "LVIII".to_owned());
    what(3999, "MMMCMXCIX".to_owned());
}
