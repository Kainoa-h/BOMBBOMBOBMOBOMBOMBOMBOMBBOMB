impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut string_stack = String::with_capacity(a.len().max(b.len()) + 1);
        let mut a_iter = a.as_bytes().iter().rev();
        let mut b_iter = b.as_bytes().iter().rev();
        let mut a_maybe = a_iter.next();
        let mut b_maybe = b_iter.next();
        let mut carry = 0;
        while a_maybe.is_some() || b_maybe.is_some() || carry == 1 {
            let ac = i32::from(*a_maybe.unwrap_or(&b'0') == b'1');
            let bc = i32::from(*b_maybe.unwrap_or(&b'0') == b'1');
            let sum = ac + bc + carry;
            let x = match sum {
                0 => (b'0', 0),
                1 => (b'1', 0),
                2 => (b'0', 1),
                3 => (b'1', 1),
                _ => panic!()
            };
            string_stack.push(x.0 as char);
            carry = x.1;
            a_maybe = a_iter.next();
            b_maybe = b_iter.next();
        }
        string_stack.chars().rev().collect()
    }
}

struct Solution {}
