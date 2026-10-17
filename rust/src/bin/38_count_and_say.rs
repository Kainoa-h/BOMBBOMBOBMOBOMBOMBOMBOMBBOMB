impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }

        let cns = Solution::count_and_say(n - 1).into_bytes();
        let mut result = String::with_capacity(cns.len() * 2);
        for arr in cns.chunk_by(|a, b| a == b) {
            result.push((b'0' + arr.len() as u8) as char);
            result.push(arr[0] as char);
        }
        result
    }
}

struct Solution {}
