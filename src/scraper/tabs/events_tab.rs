use chrono::{DateTime, FixedOffset};
use headless_chrome::browser::tab::Tab as TabEngine;
use std::sync::Arc;

pub use crate::models::contestant_odds::ContestantOdds;

use super::tab::{AsTab, Tab};

pub struct EventsTab {
    pub tab: Tab,
}

pub trait AsEventsTab: AsTab {
    fn get_events_tab(&self) -> &EventsTab;

    fn scrape_event(
        &self,
        venue_name: &str,
        event_time: DateTime<FixedOffset>,
    ) -> Result<Vec<ContestantOdds>, EventsTabError>;
}

#[derive(Debug)]
pub enum EventsTabError {
    BadScrape,
}

impl EventsTab {
    pub fn new(tab_engine: Arc<TabEngine>) -> Self {
        Self {
            tab: Tab {
                tab_engine: tab_engine,
            },
        }
    }
}
