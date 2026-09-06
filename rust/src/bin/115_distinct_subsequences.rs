impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.into_bytes();
        let t = t.into_bytes();
        let first_char = t[0];
        fn combinations(s: &[u8], t: &[u8]) -> i32 {
            println!("- s: {:?}, t: {:?}", str::from_utf8(s), str::from_utf8(t));
            if t.len() == 1 {
                let x = s.contains(&t[0]) as i32; 
                println!("- term {:?}", x);
                return x;
            }

            if s.len() == 1 {
                println!("- term none");
                return 0;
            }

            let mut calls = 0;
            for (idx, &c) in s.iter().enumerate() {
                if c == t[0] {
                    calls += combinations(&s[idx + 1..], &t[1..]);
                }
            }
            calls
        }

        let mut distinct = 0;
        for (idx, &c) in s.iter().enumerate() {
            if c == first_char {
                println!("\n\ncall 1:");
                distinct += combinations(&s[idx..], &t);
            }
        }

        distinct
    }
}

struct Solution {}

fn main() {
    assert_eq!(
        5,
        Solution::num_distinct("babgbag".to_owned(), "bag".to_owned())
    );
    assert_eq!(
        3,
        Solution::num_distinct("rabbbit".to_owned(), "rabbit".to_owned())
    );
}
