impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        for subpath in path.split('/') {
            match subpath {
                ".." => { stack.pop(); },
                "." | "" => {}
                _ => stack.push(subpath)
                
            }
        }
        format!("/{}", stack.join("/"))
    }
}

struct Solution {}

fn main() {
    Solution::simplify_path("".to_string());
}
