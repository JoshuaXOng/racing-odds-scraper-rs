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

impl EventsTab {
    pub fn new(tab_engine: Arc<TabEngine>) -> Self {
        Self {
            tab: Tab { tab_engine },
        }
    }
}

//
// Events Tab Errors.
//

#[derive(Debug)]
pub enum EventsTabError {
    General(Option<Box<dyn std::error::Error + 'static>>, String),
}

impl std::fmt::Display for EventsTabError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EventsTabError::General(_, message) => write!(formatter, "{}", message),
        }
    }
}

impl std::error::Error for EventsTabError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EventsTabError::General(source, _) => source.as_deref(),
        }
    }
}

//
// End Events Tab Errors.
//
