use crate::pages::page::{Page, AsPage};

pub struct BetfairPage {
  pub page: Page
}

impl BetfairPage {
}

impl AsPage for BetfairPage {
  fn get_page(&self) -> &Page {
    &self.page
  }
}

pub trait AsBetfairPage: AsPage {
  fn get_betfair_page(&self) -> &BetfairPage;
}

impl AsBetfairPage for BetfairPage {
  fn get_betfair_page(&self) -> &BetfairPage {
    &self
  }
}