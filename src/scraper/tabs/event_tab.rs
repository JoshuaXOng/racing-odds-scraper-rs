use std::sync::Arc;
use headless_chrome::browser::tab::Tab as TabEngine;

pub use crate::models::contestant_odds::ContestantOdds;

use super::tab::AsTab;

pub struct EventTab {
  pub tab_engine: Arc<TabEngine>,
}

pub trait AsEventTab: AsTab {
  fn get_event_tab(&self) -> &EventTab;

  fn scrape_event(&self) -> Result<Vec<ContestantOdds>, EventTabError>;
}

pub enum EventTabError {
  BadScrape
}