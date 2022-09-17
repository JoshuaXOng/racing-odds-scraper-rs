use std::sync::Arc;

use headless_chrome::Tab as TabEngine;

pub use crate::tabs::tab::{Tab, AsTab};
pub use crate::tabs::schedule_tab::ScheduleTab;
pub use crate::tabs::events_tab::EventsTab;

pub struct BetfairTab {
  pub tab: Tab,
  pub schedule_tab: ScheduleTab,
  pub events_tab: EventsTab,
}

impl BetfairTab {
  pub fn new(tab_engine: Arc<TabEngine>) -> Self {
    Self {
      tab: Tab { tab_engine: tab_engine.clone() },
      schedule_tab: ScheduleTab { tab_engine: tab_engine.clone() },
      events_tab: EventsTab { tab_engine: tab_engine.clone() },
    }
  }    
}

impl AsTab for BetfairTab {
  fn get_tab(&self) -> &Tab {
    &self.tab
  }
}
