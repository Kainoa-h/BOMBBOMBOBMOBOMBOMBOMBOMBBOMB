struct Solution {}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let base = &strs[0];
        let mut to = 0;
        'outer: for i in 0..base.len() {
            for s in strs.iter().skip(1) {
                if base.as_bytes().get(i) != s.as_bytes().get(i) {
                    break 'outer;
                }
            }
            to += 1;
        }
        base[0..to].to_owned()
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_common_prefix(vec![
            "flower".to_owned(),
            "flow".to_owned(),
            "flight".to_owned(),
        ])
    );
    println!(
        "{}",
        Solution::longest_common_prefix(vec![
            "flower".to_owned(),
            "car".to_owned(),
            "dog".to_owned(),
        ])
    );
    println!(
        "{}",
        Solution::longest_common_prefix(vec![
            "flower".to_owned(),
            "flower".to_owned(),
            "flow".to_owned(),
        ])
    );
    println!(
        "{}",
        Solution::longest_common_prefix(vec![
            "".to_owned(),
            "flow".to_owned(),
            "flight".to_owned(),
        ])
    );
    println!(
        "{}",
        Solution::longest_common_prefix(vec!["flow".to_owned(), "".to_owned(),])
    );
}
