use super::betfair_tab::BetfairTab;
use crate::{tabs::event_tab::{EventTab, AsEventTab, ContestantOdds, EventTabError}, models::odds::Odds};

impl AsEventTab for BetfairTab {
  fn get_event_tab(&self) -> &EventTab {
    &self.event_tab
  }

  fn scrape_event(&self) -> Result<Vec<ContestantOdds>, EventTabError> {
    Ok(vec![ContestantOdds {
      contestant: String::from("stub"),
      backing_odds: vec![Odds::OddsWoLiquidity(1337.0)],
      laying_odds: vec![Odds::OddsWoLiquidity(1337.0)],
    }])
  }
}
