impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        s.chars()
            .scan(0, |depth, c| {
                Some((
                    c,
                    if c == '(' {
                        *depth += 1;
                        *depth > 1
                    } else {
                        *depth -= 1;
                        *depth > 0
                    },
                ))
            })
            .filter_map(|(c, keep)| keep.then_some(c))
            .collect()
    }
}

struct Solution {}
