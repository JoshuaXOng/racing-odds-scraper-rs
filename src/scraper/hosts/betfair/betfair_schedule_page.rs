use std::sync::Arc;

use chromiumoxide::Page as PageEngine;

use crate::pages::page::{Page, AsPage};
use crate::pages::schedule_page::{SchedulePage, AsSchedulePage};
use super::betfair_page::BetfairPage;
pub use super::betfair_page::{AsBetfairPage};

pub struct BetfairSchedulePage<'a> {
  pub betfair_page: BetfairPage,
  pub schedule_page: SchedulePage<'a>
}

impl <'a> BetfairSchedulePage<'a> {
  pub fn new(page_engine: PageEngine, target_url: &'a String) -> Self {
    BetfairSchedulePage {
      betfair_page: BetfairPage {
        page: Page { engine: Arc::new(page_engine), target_url: Arc::new(Some(*target_url)) }
      },
      schedule_page: SchedulePage {
        page: Page { engine: Arc::new(page_engine), target_url: Arc::new(Some(*target_url)) },
        target_url: Arc::new(target_url)
      }
    }
  }
}

impl<'a> AsPage for BetfairSchedulePage<'a> {
  fn get_page(&self) -> &Page {
    self.betfair_page.get_page()
  }
}

impl<'a> AsBetfairPage for BetfairSchedulePage<'a> {
  fn get_betfair_page(&self) -> &BetfairPage {
    &self.betfair_page
  }
}

impl<'a> AsSchedulePage for BetfairSchedulePage<'a> {
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

