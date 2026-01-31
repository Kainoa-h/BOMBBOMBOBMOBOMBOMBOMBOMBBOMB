struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut rows: Vec<Vec<char>> = (0..num_rows).map(|_| Vec::new()).collect();
        let mut row_idx = 0;
        let mut forwards = true;
        for c in s.chars() {
            rows[row_idx].push(c);
            if forwards {
                if row_idx as i32 == num_rows - 1 {
                    row_idx -= 1;
                    forwards = false;
                    continue;
                }
                row_idx += 1;
                continue;
            }
            if row_idx == 0 {
                row_idx = 1;
                forwards = true;
                continue;
            }
            row_idx -= 1;
        }
        rows.into_iter().flatten().collect()
    }
}

fn what(s: &str, num_rows: i32, ans: &str) {
    println!("{}", s);
    assert_eq!(Solution::convert(s.to_owned(), num_rows), ans);
}

fn main() {
    what("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR");
}
