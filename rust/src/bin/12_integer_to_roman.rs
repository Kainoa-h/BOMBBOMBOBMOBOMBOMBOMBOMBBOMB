struct Solution {}
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut maps = [
            |n| match n {
                1..=3 => "I".repeat(n),
                4 => "IV".to_owned(),
                5..=8 => format!("V{}", "I".repeat(n - 5)),
                9 => "IX".to_owned(),
                _ => "".to_owned(),
            },
            |n| match n {
                1..=3 => "X".repeat(n),
                4 => "XL".to_owned(),
                5..=8 => format!("L{}", "X".repeat(n - 5)),
                9 => "XC".to_owned(),
                _ => "".to_owned(),
            },
            |n| match n {
                1..=3 => "C".repeat(n),
                4 => "CD".to_owned(),
                5..=8 => format!("D{}", "C".repeat(n - 5)),
                9 => "CM".to_owned(),
                _ => "".to_owned(),
            },
            |n| match n {
                1..=3 => "M".repeat(n),
                _ => "".to_owned(),
            },
        ]
        .iter();

        let mut current_map;
        let mut units: Vec<String> = Vec::new();
        while num > 0 {
            current_map = maps.next().unwrap();
            let x = num % 10;
            units.push(current_map(x as usize));
            num = (num - x) / 10;
        }
        units.into_iter().rev().collect()
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
