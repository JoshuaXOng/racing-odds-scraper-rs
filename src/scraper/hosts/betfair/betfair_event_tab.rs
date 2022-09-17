use super::betfair_tab::BetfairTab;
use crate::{tabs::events_tab::{EventsTab, AsEventsTab, ContestantOdds, EventsTabError}, models::odds::Odds};

impl AsEventsTab for BetfairTab {
  fn get_events_tab(&self) -> &EventsTab {
    &self.events_tab
  }

  fn scrape_event(&self) -> Result<Vec<ContestantOdds>, EventsTabError> {
    Ok(vec![ContestantOdds {
      contestant: String::from("stub"),
      backing_odds: vec![Odds::OddsWoLiquidity(1337.0)],
      laying_odds: vec![Odds::OddsWoLiquidity(1337.0)],
    }])
  }
}
