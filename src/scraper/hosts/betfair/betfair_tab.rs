use std::sync::Arc;

use headless_chrome::Tab as TabEngine;

pub use crate::tabs::events_tab::EventsTab;
pub use crate::tabs::schedule_tab::ScheduleTab;
pub use crate::tabs::tab::{AsTab, Tab};

pub struct BetfairTab {
    pub tab: Tab,
}

pub trait AsBetfairTab: AsTab {
    fn get_betfair_tab(&self) -> &BetfairTab;
}

impl AsTab for BetfairTab {
    fn get_tab(&self) -> &Tab {
        &self.tab
    }
}

impl BetfairTab {
    pub fn new(tab_engine: Arc<TabEngine>) -> Self {
        Self {
            tab: Tab { tab_engine },
        }
    }
}
