use crate::pages::page::{Page, AsPage};
use crate::pages::schedule_page::{SchedulePage, AsSchedulePage};
use super::betfair_page::BetfairPage;
pub use super::betfair_page::{AsBetfairPage};

pub struct BetfairSchedulePage {
  pub betfair_page: BetfairPage,
  pub schedule_page: SchedulePage
}

impl BetfairSchedulePage {
}

impl AsPage for BetfairSchedulePage {
  fn get_page(&self) -> &Page {
    self.betfair_page.get_page()
  }
}

impl AsBetfairPage for BetfairSchedulePage {
  fn get_betfair_page(&self) -> &BetfairPage {
    &self.betfair_page
  }
}

impl AsSchedulePage for BetfairSchedulePage {
  fn get_schedule_page(&self) -> &SchedulePage {
    &self.schedule_page
  }

  fn perform_scrape(&self) -> {
    self.get_schedule_page().page.evaluate_function("() => {
      const venueRows = document.getElementsByClass(\"race-list\");      
      venueRows.map(vRow => {
        let events = [];

        vRow.forEach(event_cell => {
          
        })
      })
    }")
  }
}

