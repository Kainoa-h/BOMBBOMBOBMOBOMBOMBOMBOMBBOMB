struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

fn main() {
    println!(
        "'{}'",
        Solution::reverse_words("  a good   example  ".to_owned())
    );
}
