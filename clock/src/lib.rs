use std::fmt;
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        while hours > 23 || hours < 0 || minutes > 59 || minutes < 0 {
            if hours < 0 {
                hours = 24 + hours;
            }
            if hours > 23 {
                hours = hours - 24;
            }
            if minutes > 59 {
                minutes = minutes - 60;
                hours = hours + 1;
            }
            if minutes < 0 {
                minutes = 60 + minutes;
                hours = hours - 1
            }
        }
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hr = if self.hours < 10 {
            format!("0{}", self.hours.to_string())
        } else {
            format!("{}", self.hours.to_string())
        };
        let mn = if self.minutes < 10 {
            format!("0{}", self.minutes.to_string())
        } else {
            format!("{}", self.minutes.to_string())
        };
        write!(f, "{}:{}", hr, mn)
    }
}

