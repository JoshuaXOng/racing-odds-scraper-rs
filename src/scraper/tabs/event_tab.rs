use std::sync::Arc;
use headless_chrome::browser::tab::Tab as TabEngine;

pub use crate::models::contestant_odds::ContestantOdds;

pub struct EventTab {
  tab_engine: Arc<TabEngine>,
}

pub trait AsEventTab {
  fn get_event_tab(&self) -> &EventTab;

  fn scrape_odds(&self) -> &Vec<ContestantOdds>;
}
