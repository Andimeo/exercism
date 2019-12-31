use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    const MINUTES_IN_HOUR: i32 = 60;
    const HOURS_IN_DAY: i32 = 24;

    fn convert(hours: i32, minutes: i32) -> Self {
        let m = minutes.rem_euclid(Clock::MINUTES_IN_HOUR);
        let h = (hours - (m - minutes) / Clock::MINUTES_IN_HOUR).rem_euclid(Clock::HOURS_IN_DAY);
        Clock { hours: h, minutes: m }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::convert(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::convert(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
