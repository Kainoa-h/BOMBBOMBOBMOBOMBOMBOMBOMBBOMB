use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::<[u32; 26], Vec<String>>::new();
        for str in strs {
            let encoded_str = str.bytes().fold([0_u32; 26], |mut state, x| {
                state[(x - b'a') as usize] += 1;
                state
            });

            map.entry(encoded_str).or_default().push(str);
        }

        let mut result = Vec::new();
        for e in map.into_values() {
            result.push(e);
        }

        result
    }
}

struct Solution {}

fn main() {
    println!(
        "{:?}",
        Solution::group_anagrams(vec![
            "eat".to_owned(),
            "tea".to_owned(),
            "tan".to_owned(),
            "ate".to_owned(),
            "nat".to_owned(),
            "bat".to_owned(),
        ])
    );

    println!(
        "ans: {:?}",
        vec![
            vec!["bat".to_owned()],
            vec!["nat".to_owned(), "tan".to_owned()],
            vec!["ate".to_owned(), "eat".to_owned(), "tea".to_owned()],
        ]
    );
}
