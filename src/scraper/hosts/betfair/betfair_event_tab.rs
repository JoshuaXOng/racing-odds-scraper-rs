use super::betfair_tab::BetfairTab;
use crate::tabs::event_tab::EventTab;

impl EventTab for BetfairTab {
  fn get_event_tab(&self) -> &EventTab {
    &self.event_tab;
  }

  fn scrape_events(&self) -> &Vec<ContestantOdds> {
    vec![ContestantOdds {
      contestant: "sdas",
      backing_odds: Odds::,
      laying_odds: Odds::,
    }]
  }
}
