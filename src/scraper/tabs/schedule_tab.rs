use std::sync::Arc;
use headless_chrome::browser::tab::Tab as TabEngine;

pub use crate::models::event::Event;

use super::tab::AsTab;

pub struct ScheduleTab {
  pub tab_engine: Arc<TabEngine>,
}

pub trait AsScheduleTab: AsTab {
  fn get_schedule_tab(&self) -> &ScheduleTab;

  fn scrape_schedule(&self) -> Result<Vec<Event>, ScheduleTabError>;
}

pub enum ScheduleTabError {
  BadScrape
}
