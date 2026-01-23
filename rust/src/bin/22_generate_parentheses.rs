struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut strings: Vec<String> = Vec::new();
        let final_len: usize = (n * 2) as usize;

        let mut stack: Vec<State> = vec![State {
            pattern: String::with_capacity(final_len),
            open: n,
            close: n,
        }];

        while let Some(mut state) = stack.pop() {
            // end state
            if state.pattern.len() == final_len {
                strings.push(state.pattern);
                continue;
            }

            // shortcut to end state
            if state.open == 0 {
                state.pattern.push_str(&")".repeat(state.close as usize));
                strings.push(state.pattern);
                continue;
            }

            // must open
            if state.open == state.close {
                state.pattern.push('(');
                state.open -= 1;
                stack.push(state);
                continue;
            }

            let mut other_state = State {
                pattern: String::with_capacity(final_len),
                open: state.open,
                close: state.close,
            };
            other_state.pattern.push_str(&state.pattern.clone());

            state.pattern.push(')');
            state.close -= 1;
            stack.push(state);

            other_state.pattern.push('(');
            other_state.open -= 1;
            stack.push(other_state);
        }

        strings
    }
}

struct State {
    pattern: String,
    open: i32,
    close: i32,
}

fn what(n: i32) {
    print!("{:?}", Solution::generate_parenthesis(n))
}

fn main() {
    what(3);
}
