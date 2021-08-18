use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut add_hours = minutes / 60;
        let mut minutes = minutes % 60;
        if minutes < 0 {
            add_hours -= 1;
            minutes = 60 + minutes;
        }

        let mut hours = (hours + add_hours) % 24;
        hours = if hours < 0 { 24 + hours } else { hours };
        Self {
            hours,
            minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

// impl ToString for Clock {
//     fn to_string(&self) -> String {
//         format!("{:02}:{:02}", self.hours, self.minutes)
//     }
// }

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl From<Clock> for String {
    fn from(c: Clock) -> Self {
        format!("{:02}:{:02}", c.hours, c.minutes)
    }
}
