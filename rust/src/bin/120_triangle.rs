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
                let left = *prev_row.get(idx - 1).unwrap_or(&i32::MAX);
                let right = *prev_row.get(idx).unwrap_or(&i32::MAX);
                *v += left.min(right);
            }
        }

        triangle.last().map_or(0, |x| *x.iter().min().unwrap_or(&0))
    }
}

struct Solution {}
