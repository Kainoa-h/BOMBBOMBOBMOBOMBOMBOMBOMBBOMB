struct Solution {}
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = [-1_i16; 128];
        let mut u_map = [false; 128];
        for i in 0..s.len() {
            let c1 = s.as_bytes()[i];
            let c2 = t.as_bytes()[i];
            let x = map.get_mut(c1 as usize).unwrap();
            let y = u_map.get_mut(c2 as usize).unwrap();

            if *x == -1 && !*y {
                *x = c2 as i16;
                *y = true;
            } else if *x == -1 && *y {
                return false;
            } else if *x != c2 as i16 {
                return false;
            }
        }
        true
    }
}

fn main() {
    // assert!(Solution::is_isomorphic("egg".to_owned(), "add".to_owned()));
    // assert!(!Solution::is_isomorphic("f11".to_owned(), "b23".to_owned()));
    // assert!(Solution::is_isomorphic(
    //     "paper".to_owned(),
    //     "title".to_owned()
    // ));
    assert!(!Solution::is_isomorphic(
        "badc".to_owned(),
        "baba".to_owned()
    ));
}
