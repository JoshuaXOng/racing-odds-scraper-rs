use std::sync::Arc;

pub use crate::models::event::Event;

use headless_chrome::{Tab as TabEngine};

use super::tab::{AsTab, Tab};

pub struct ScheduleTab {
  pub tab: Tab,
}

pub trait AsScheduleTab: AsTab {
  fn get_schedule_tab(&self) -> &ScheduleTab;

  fn scrape_schedule(&self) -> Result<Vec<Event>, ScheduleTabError>;
}

#[derive(Debug)]
pub enum ScheduleTabError {
  BadScrape
}

impl ScheduleTab {
  pub fn new(tab_engine: Arc<TabEngine>) -> Self {
    Self {
      tab: Tab { 
        tab_engine: tab_engine
      },
    }
  }
}
