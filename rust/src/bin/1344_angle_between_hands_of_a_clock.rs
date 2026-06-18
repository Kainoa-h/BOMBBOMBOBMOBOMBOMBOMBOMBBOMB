static HOUR_ANGLE_INCR: f64 = 30.0;
static SUB_HOUR_ANGLE_INCR: f64 = 0.5;
static MIN_ANGLE_INCR: f64 = 6.0;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let hour = if hour == 12 { 0.0 } else { hour as f64 };
        let minutes = if minutes == 60 { 0.0 } else { minutes as f64 };
        let hour_angle = (hour * HOUR_ANGLE_INCR) + (minutes * SUB_HOUR_ANGLE_INCR);
        let min_angle = minutes * MIN_ANGLE_INCR;
        let diff = (hour_angle - min_angle).abs();
        diff.min(360.0 - diff)
    }
}

struct Solution {}
