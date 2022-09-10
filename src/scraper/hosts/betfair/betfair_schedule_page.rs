use std::sync::Arc;

use chromiumoxide::Page as PageEngine;

use crate::pages::page::{Page, AsPage};
use crate::pages::schedule_page::{SchedulePage, AsSchedulePage};
use super::betfair_page::BetfairPage;
pub use super::betfair_page::{AsBetfairPage};

pub struct BetfairSchedulePage {
  pub betfair_page: BetfairPage,
  pub schedule_page: SchedulePage
}

impl BetfairSchedulePage {
  pub fn new(page_engine: PageEngine, target_url: String) -> Self {
    let page_engine = Arc::new(page_engine);
    let target_url = Arc::new(target_url);

    BetfairSchedulePage {
      betfair_page: BetfairPage {
        page: Page { engine: page_engine.clone(), target_url: Some(target_url.clone()) }
      },
      schedule_page: SchedulePage {
        page: Page { engine: page_engine.clone(), target_url: Some(target_url.clone()) },
        target_url: target_url.clone()
      }
    }
  }
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

  fn perform_scrape(&self) -> i32 {
    // self.get_schedule_page().page.evaluate_function("() => {
    //   const venueRows = document.getElementsByClass(\"race-list\");      
    //   venueRows.map(vRow => {
    //     let events = [];

    //     vRow.forEach(event_cell => {
          
    //     })
    //   })
    // }");
    3
  }
}

