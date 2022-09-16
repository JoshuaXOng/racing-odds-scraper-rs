use std::fmt;

use chrono::NaiveDateTime;

pub struct Event {
  pub venue_name: String,
  pub planned_start_time: NaiveDateTime,
  pub has_started: bool,
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