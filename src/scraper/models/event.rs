use std::fmt;

use chrono::{DateTime, FixedOffset};

#[derive(Debug)]
pub struct Event {
    pub venue_name: String,
    pub planned_start_time: DateTime<FixedOffset>,
}

impl fmt::Display for Event {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{{ venue_name={}, planned_start_time={} }}",
            self.venue_name, self.planned_start_time
        )
    }
}
