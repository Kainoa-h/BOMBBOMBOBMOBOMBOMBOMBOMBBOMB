impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 1 {
            return triangle[0][0];
        }

        for row_idx in 1..triangle.len() {
            let (prev_rows, next_rows) = triangle.split_at_mut(row_idx);
            let prev_row = prev_rows.last().unwrap();
            let row = next_rows.first_mut().unwrap();
            for (idx, v) in row.iter_mut().enumerate() {
                *v += match idx {
                    0 => prev_row[0],
                    idx if idx == prev_row.len() => prev_row[idx - 1],
                    idx => prev_row[idx].min(prev_row[idx - 1]),
                }
            }
        }

        triangle
            .last()
            .and_then(|x| x.iter().min())
            .copied()
            .unwrap_or(0)
    }
}

struct Solution {}
