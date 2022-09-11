use chrono::NaiveDateTime;

pub struct Event {
  pub venue_name: String,
  pub planned_start_time: NaiveDateTime,
  pub has_started: bool,
}
