impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let (m , n) = (m as usize, n as usize);
        let mut result = Vec::with_capacity(m);
        result.push(".".repeat(n));
        for _ in 1..m {
            let mut s = "#".repeat(n-1);
            s.push('.');
            result.push(s);
        }

        result
    }
}

struct Solution {}
