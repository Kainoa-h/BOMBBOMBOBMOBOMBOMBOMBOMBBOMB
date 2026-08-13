use std::iter;

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let mut sb = s.into_bytes();
        let mut dep_list = (0..sb.len())
            .zip(iter::repeat(1))
            .collect::<Vec<(usize, i32)>>();

        for (idx, w) in sb.windows(2).enumerate() {
            if w[0] == w[1] {
                dep_list[idx + 1].0 = idx;
            }
        }

        for i in (1..dep_list.len()).rev() {
            let right = dep_list[i];
            if right.0 == i - 1 {
                dep_list[i - 1].1 += right.1;
            }
        }

        let mut max_list = Vec::with_capacity(query_characters.len());
        for (&c, i) in query_characters.as_bytes().iter().zip(query_indices) {
            let idx = i as usize;

            if c == sb[idx] {
                max_list.push(if let Some(&p) = max_list.last() {
                    p
                } else {
                    dep_list.iter().map(|x| x.1).max().unwrap_or(0)
                });
                continue;
            }

            if dep_list[idx].0 == idx {
                dep_list[idx].1 = 1;
                if let Some(next) = dep_list.get_mut(idx + 1) {
                    next.0 = idx + 1;
                }
            } else {
                let len = dep_list[idx].1;
                dep_list[idx].0 = idx;
                dep_list[idx].1 = 1;
                if let Some(next) = dep_list.get_mut(idx + 1) {
                    next.0 = idx + 1;
                }

                let mut prev_idx = idx - 1;
                while let Some(prev) = dep_list.get_mut(prev_idx) {
                    prev.1 -= len;
                    if prev.0 == prev_idx {
                        break;
                    }
                    prev_idx -= 1;
                }
            }

            sb[idx] = c;
            if let Some(&next_char) = sb.get(idx + 1)
                && next_char == c
            {
                let next_dep = &mut dep_list[idx + 1];
                next_dep.0 = idx;
                dep_list[idx].1 += next_dep.1;
            }

            if idx != 0
                && let Some(&prev_char) = sb.get(idx - 1)
                && prev_char == c
            {
                dep_list[idx].0 = idx - 1;
                let len = dep_list[idx].1;
                let mut prev_idx = idx - 1;
                while let Some(prev_dep) = dep_list.get_mut(prev_idx) {
                    prev_dep.1 += len;
                    if prev_dep.0 == prev_idx {
                        break;
                    }
                    prev_idx -= 1;
                }
            }

            max_list.push(dep_list.iter().max_by_key(|x| x.1).unwrap().1);
        }

        max_list
    }
}

struct Solution {}
