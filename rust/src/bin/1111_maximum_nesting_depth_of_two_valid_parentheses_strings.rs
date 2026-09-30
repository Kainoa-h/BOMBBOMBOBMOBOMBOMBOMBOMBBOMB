impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        seq.bytes()
            .scan(0, |depth, byte| match byte {
                b'(' => {
                    let bin = *depth % 2;
                    *depth += 1;
                    Some(bin)
                }
                b')' => {
                    *depth -= 1;
                    Some(*depth % 2)
                }
                _ => unreachable!(),
            })
            .collect()
    }
}

struct Solution {}
