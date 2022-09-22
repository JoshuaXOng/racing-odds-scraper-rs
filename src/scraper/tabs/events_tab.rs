use std::sync::Arc;
use chrono::{FixedOffset, DateTime};
use headless_chrome::browser::tab::Tab as TabEngine;

pub use crate::models::contestant_odds::ContestantOdds;

use super::tab::AsTab;

pub struct EventsTab {
  pub tab_engine: Arc<TabEngine>,
}

pub trait AsEventsTab: AsTab {
  fn get_events_tab(&self) -> &EventsTab;

  fn scrape_event(&self, venue_name: &str, event_time: DateTime<FixedOffset>) -> Result<Vec<ContestantOdds>, EventsTabError>;
}

#[derive(Debug)]
pub enum EventsTabError {
  BadScrape
}

impl EventsTab {
  pub fn new(tab_engine: Arc<TabEngine>) -> Self {
    Self { tab_engine }
  }
}