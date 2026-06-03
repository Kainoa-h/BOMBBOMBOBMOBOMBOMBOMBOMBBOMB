struct Solution {}
impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let earliest_land_finish = land_start_time
            .iter()
            .zip(land_duration.iter().by_ref())
            .map(|(s, d)| s + d)
            .min()
            .unwrap();
        let earliest_land_then_water_finish = water_start_time
            .iter()
            .zip(water_duration.iter().by_ref())
            .map(|(s, d)| earliest_land_finish.max(*s) + d)
            .min()
            .unwrap();

        let earliest_water_finish = water_start_time
            .iter()
            .zip(water_duration.iter().by_ref())
            .map(|(s, d)| s + d)
            .min()
            .unwrap();

        let earliest_water_then_land_finish = land_start_time
            .iter()
            .zip(land_duration.iter().by_ref())
            .map(|(s, d)| earliest_water_finish.max(*s) + d)
            .min()
            .unwrap();
        earliest_water_then_land_finish.min(earliest_land_then_water_finish)
    }
}
