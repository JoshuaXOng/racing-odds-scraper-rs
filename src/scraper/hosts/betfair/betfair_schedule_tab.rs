use super::betfair_tab::BetfairTab;
use crate::tabs::schedule_tab::{ScheduleTab, Event};

impl ScheduleTab for BetfairTab {
  fn get_schedule_tab(&self) -> &ScheduleTab {
    &self.schedule_tab;
  }

  fn scrape_events(&self) -> &Vec<Event> {
    vec![Event {
      venue_name: "sd",
      planned_start_time: NaiveDate::from_ymd(2016, 7, 8),
      has_started: true,
    }]
  }
}
