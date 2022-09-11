use chrono::NaiveDateTime;

use super::betfair_tab::BetfairTab;
use crate::tabs::schedule_tab::{ScheduleTab, Event, AsScheduleTab};

impl AsScheduleTab for BetfairTab {
  fn get_schedule_tab(&self) -> &ScheduleTab {
    &self.schedule_tab
  }

  fn scrape_events(&self) -> Vec<Event> {
    vec![Event {
      venue_name: String::from("sd"),
      planned_start_time: NaiveDateTime::from_timestamp(100, 10),
      has_started: true,
    }]
  }
}
