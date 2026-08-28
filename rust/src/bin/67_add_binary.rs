impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut string_stack = String::with_capacity(a.len().max(b.len()) + 1);
        let mut a = a.bytes().rev().peekable();
        let mut b = b.bytes().rev().peekable();
        let mut carry = 0;

        while a.peek().is_some() || b.peek().is_some() || carry == 1 {
            let sum = carry + a.next().map_or(0, |b| b - b'0') + b.next().map_or(0, |b| b - b'0');
            string_stack.push((b'0' + sum % 2) as char);
            carry = sum / 2;
        }

        string_stack.chars().rev().collect()
    }
}

struct Solution {}
