impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut parity = 0;
        let mut extra_lifes = 0;
        let mut extra_lifes_since = 0;
        for c in s.chars() {
            match c {
                '(' => parity += 1,
                ')' => {
                    parity -= 1;
                    if parity < 0 {
                        if extra_lifes > 0 {
                            parity = 0;
                            extra_lifes -= 1;
                        } else {
                            return false;
                        }
                    }
                },
                _ => {
                    extra_lifes += 1;
                    extra_lifes_since += 1;
                }
            }
            if parity == 0 {
                extra_lifes_since = 0;
            }
        }
        parity == 0 || extra_lifes_since >= parity
    }
}

struct Solution {}
