impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let (m , n) = (m as usize, n as usize);
        let mut result = Vec::with_capacity(m);
        result.push(".".repeat(n));
        let s = format!("{}.", "#".repeat(n-1));
        result.resize(m, s);
        result
    }
}

struct Solution {}
