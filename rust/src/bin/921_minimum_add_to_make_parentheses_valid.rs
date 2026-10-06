impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let state = s.chars().fold((0, 0, 0), |mut state, c| {
            match c {
                '(' => state.0 += 1,
                _ => state.1 += 1,
            }
            if state.1 > state.0 {
                state.2 += 1;
                state = (0, 0, state.2);
            }
            state
        });
        state.2 + state.0 - state.1
    }
}

struct Solution {}
