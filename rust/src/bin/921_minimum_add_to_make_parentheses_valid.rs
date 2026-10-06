impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let state = s.chars().fold((0, 0), |mut state, c| {
            match c {
                '(' => state.0 += 1,
                _ => {
                    if state.0 > 0 {
                        state.0 -= 1;
                    } else {
                        state.1 += 1;
                    }
                }
            }
            state
        });
        state.0 + state.1
    }
}

struct Solution {}
