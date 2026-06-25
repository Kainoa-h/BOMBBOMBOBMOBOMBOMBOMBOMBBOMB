impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval; 1];
        }

        let mut result = Vec::<Vec<i32>>::new();

        let mut not_merged = true;

        if new_interval[0] < intervals[0][0] {
            if new_interval[1] >= intervals[0][0] {
                intervals[0][0] = new_interval[0];
                intervals[0][1] = intervals[0][1].max(new_interval[1]);
            } else {
                result.push(new_interval.clone());
            }
            not_merged = false;
        }

        for mut intv in intervals.into_iter() {
            if not_merged {
                if ((new_interval[0] >= intv[0] && new_interval[0] <= intv[1])
                    || (new_interval[1] >= intv[0] && new_interval[1] <= intv[1])
                    || (intv[0] >= new_interval[0] && intv[0] <= new_interval[1])
                    || (intv[1] >= new_interval[0] && intv[1] <= new_interval[1]))
                {
                    intv[0] = intv[0].min(new_interval[0]);
                    intv[1] = intv[1].max(new_interval[1]);
                    not_merged = false;
                } else if new_interval[1] < intv[0] {
                    result.push(new_interval.clone());
                    not_merged = false;
                }
            }

            if let Some(prev) = result.last_mut()
                && prev[1] >= intv[0]
            {
                prev[1] = prev[1].max(intv[1]);
            } else {
                result.push(intv);
            }
        }

        if not_merged {
            result.push(new_interval);
        }

        result
    }
}

struct Solution {}
