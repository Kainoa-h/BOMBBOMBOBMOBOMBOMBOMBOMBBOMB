impl Solution {
    pub fn min_operations(s: String) -> i32 {
        if s.len() <= 1 {
            return 0;
        }
        let bytes = s.as_bytes();

        if s.len() == 2 {
            return if bytes[0] <= bytes[1] { 0 } else { -1 };
        }

        let mut bucket = [0; 256];
        let mut sorted = true;
        for i in 0..bytes.len() {
            bucket[bytes[i] as usize] += 1;
            if i > 0 && bytes[i - 1] > bytes[i] {
                sorted = false;
            }
        }
        if sorted {
            return 0;
        }

        let min = bucket[b'a' as usize..].iter().position(|&b| b > 0).unwrap() as u8 + b'a';
        let max = bucket[b'a' as usize..=b'z' as usize]
            .iter()
            .rposition(|&b| b > 0)
            .unwrap() as u8
            + b'a';

        if min == bytes[0] || max == bytes[bytes.len() - 1] {
            return 1;
        }

        if max == bytes[0]
            && min == bytes[bytes.len() - 1]
            && bucket[min as usize] == 1
            && bucket[max as usize] == 1
        {
            return 3;
        }

        2
    }
}

struct Solution {}
