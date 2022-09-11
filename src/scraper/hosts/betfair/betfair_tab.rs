pub use crate::tabs::tab::{Tab, AsTab};
pub use crate::tabs::schedule_tab::ScheduleTab;
pub use crate::tabs::event_tab::EventTab;

pub struct BetfairTab {
  pub tab: Tab,
  pub schedule_tab: ScheduleTab,
  pub event_tab: EventTab,
}

impl AsTab for BetfairTab {
  fn get_tab(&self) -> &Tab {
    &self.tab
  }
}
