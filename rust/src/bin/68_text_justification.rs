struct Solution {}
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut start_idx = 0;
        let mut end_idx = 0;
        let mut len_so_far = 0;
        let mut line_list: Vec<String> = Vec::new();
        for w in &words {
            if w.len() + len_so_far + end_idx - start_idx <= max_width as usize {
                len_so_far += w.len();
                end_idx += 1;
                continue;
            }
            // doesn't fit
            let count = end_idx - start_idx;
            let spaces = count - 1;
            let extra_characters = max_width - len_so_far as i32;

            let space_size = extra_characters.checked_div(spaces as i32).unwrap_or(0);
            let padding_space = extra_characters.checked_rem(spaces as i32).unwrap_or(0);

            let mut line = String::with_capacity(max_width as usize);
            line.push_str(&words[start_idx]);
            line.push_str(&" ".repeat(padding_space as usize));
            for i in start_idx + 1..end_idx {
                line.push_str(&" ".repeat(space_size as usize));
                line.push_str(&words[i]);
            }
            line_list.push(line);

            start_idx = end_idx;
            len_so_far = w.len();
            end_idx += 1;
        }
        let mut line = String::with_capacity(max_width as usize);
        let mut chars = 0;
        for idx in start_idx..words.len() {
            chars += words[idx].len();
            line.push_str(&words[idx]);
            line.push_str(&" ".repeat(max_width as usize - chars));
        }
        line_list.push(line);

        line_list
    }
}

fn what(words: Vec<String>, max_width: i32) {
    for l in Solution::full_justify(words, max_width) {
        println!("'{}'", l);
    }
}

fn main() {
    what(
        vec![
            "This".to_owned(),
            "is".to_owned(),
            "an".to_owned(),
            "example".to_owned(),
            "of".to_owned(),
            "text".to_owned(),
            "justification.".to_owned(),
        ],
        16,
    );
}
