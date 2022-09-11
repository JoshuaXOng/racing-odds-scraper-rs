use std::sync::Arc;
use headless_chrome::browser::tab::Tab as TabEngine;

pub use crate::models::event::Event;

pub struct ScheduleTab {
  tab_engine: Arc<TabEngine>,
}

pub trait AsScheduleTab {
  fn get_schedule_tab(&self) -> &ScheduleTab;

  fn scrape_events(&self) -> &Vec<Event>;
}
