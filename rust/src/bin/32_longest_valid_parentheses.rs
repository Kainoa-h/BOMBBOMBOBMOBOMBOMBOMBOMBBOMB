impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        fn search<'a, I>(iter: I, open: u8) -> i32
        where
            I: Iterator<Item = &'a u8>,
        {
            iter.fold((0, 0, 0), |mut acc, &x| {
                if x == open {
                    acc.0 += 1;
                } else {
                    acc.1 += 1;
                }
                if acc.0 == acc.1 {
                    acc.2 = acc.2.max(acc.0 * 2);
                } else if acc.0 < acc.1 {
                    acc = (0, 0, acc.2);
                }
                acc
            })
            .2
        }

        i32::max(
            search(s.as_bytes().iter(), b'('),
            search(s.as_bytes().iter().rev(), b')')
        )
    }
}

struct Solution {}
