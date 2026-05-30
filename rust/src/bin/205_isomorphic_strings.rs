struct Solution {}
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_map = [None::<u8>; 128];
        let mut t_check = [None::<u8>; 128];

        for (c1, c2) in s.bytes().zip(t.bytes()) {
            let e1 = s_map.get_mut(c1 as usize).unwrap().get_or_insert(c2);
            if *e1 != c2 {
                return false;
            }

            let e2 = t_check.get_mut(c2 as usize).unwrap().get_or_insert(c1);
            if *e2 != c1 {
                return false;
            }
        }

        true
    }
}

fn main() {
    assert!(Solution::is_isomorphic("egg".to_owned(), "add".to_owned()));
    assert!(!Solution::is_isomorphic("f11".to_owned(), "b23".to_owned()));
    assert!(Solution::is_isomorphic(
        "paper".to_owned(),
        "title".to_owned()
    ));
    assert!(!Solution::is_isomorphic(
        "badc".to_owned(),
        "baba".to_owned()
    ));
}
