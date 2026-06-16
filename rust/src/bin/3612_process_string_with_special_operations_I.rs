impl Solution {
    pub fn process_str(s: String) -> String {
        let mut result = String::new();

        for c in s.chars() {
            match c {
                '*' => {
                    result.pop();
                }
                '#' => {
                    result += &result.clone();
                }
                '%' => {
                    result = result.chars().rev().collect::<String>();
                }
                _ => {
                    result.push(c);
                }
            }
        }

        result
    }
}

struct Solution {}
