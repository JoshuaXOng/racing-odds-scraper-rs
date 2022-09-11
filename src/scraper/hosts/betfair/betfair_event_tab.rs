use super::betfair_tab::BetfairTab;
use crate::{tabs::event_tab::{EventTab, AsEventTab, ContestantOdds}, models::odds::Odds};

impl AsEventTab for BetfairTab {
  fn get_event_tab(&self) -> &EventTab {
    &self.event_tab
  }

  fn scrape_odds(&self) -> Vec<ContestantOdds> {
    vec![ContestantOdds {
      contestant: String::from("sdas"),
      backing_odds: vec![Odds::OddsWoLiquidity(10.0)],
      laying_odds: vec![Odds::OddsWoLiquidity(11.0)],
    }]
  }
}
