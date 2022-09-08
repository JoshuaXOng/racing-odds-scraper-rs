use crate::pages::page::{Page, AsPage};
pub use super::betfair_page::{BetfairPage};

pub struct BetfairSchedulePage {
  pub page: Page
}

impl AsPage for BetfairSchedulePage {
  fn get_page(&self) -> &Page {
    &self.page
  }
}

impl BetfairPage for BetfairSchedulePage {
}

impl BetfairSchedulePage {
  fn goto_schedule_page() {}
}
