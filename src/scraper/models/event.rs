use std::fmt;

use chrono::{DateTime, FixedOffset};

pub struct Event {
  pub venue_name: String,
  pub planned_start_time: DateTime<FixedOffset>,
  pub has_started: bool,
}

impl Event {
  fn is_within(&self, a: DateTime<FixedOffset>, b: u32) {
    
  }
}

impl fmt::Display for Event {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(
      formatter, 
      "{{ venue_name={}, planned_start_time={}, has_started={} }}", 
      self.venue_name, self.planned_start_time, self.has_started
    )
  }
}