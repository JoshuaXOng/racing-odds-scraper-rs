use std::sync::Arc;

pub use crate::models::event::Event;

use headless_chrome::Tab as TabEngine;

use super::tab::{AsTab, Tab};

pub struct ScheduleTab {
    pub tab: Tab,
}

pub trait AsScheduleTab: AsTab {
    fn get_schedule_tab(&self) -> &ScheduleTab;

    fn scrape_schedule(&self) -> Result<Vec<Event>, ScheduleTabError>;
}

impl ScheduleTab {
    pub fn new(tab_engine: Arc<TabEngine>) -> Self {
        Self {
            tab: Tab {
                tab_engine: tab_engine,
            },
        }
    }
}

//
// Schedule Tab Errors.
//

#[allow(dead_code)]
#[derive(Debug)]
pub enum ScheduleTabError {
    General(Option<Box<dyn std::error::Error + 'static>>, String),
}

impl std::fmt::Display for ScheduleTabError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScheduleTabError::General(_, message) => write!(formatter, "{}", message),
        }
    }
}

impl std::error::Error for ScheduleTabError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ScheduleTabError::General(source, _) => source.as_deref(),
        }
    }
}

//
// End Schedule Tab Errors.
//
