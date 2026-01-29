struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = |c: char| match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        let mut total = 0;
        let mut prev = 0;
        for (_, c) in s.char_indices() {
            let x = map(c);
            total += x;
            if prev < x {
                total -= 2 * prev;
            }
            prev = x;
        }
        total
    }
}

fn what(s: String, ans: i32) {
    println!("{}", s);
    assert_eq!(Solution::roman_to_int(s), ans);
}

fn main() {
    what("III".to_owned(), 3);
    what("IV".to_owned(), 4);
    what("LVIII".to_owned(), 58);
    what("MMMCMXCIX".to_owned(), 3999);
}
