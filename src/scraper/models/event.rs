use chrono::NaiveDateTime;

pub struct Event {
  venue_name: String,
  planned_start_time: NaiveDateTime,
  has_started: bool,
}
