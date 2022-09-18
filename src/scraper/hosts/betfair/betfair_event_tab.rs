use chrono::{DateTime, FixedOffset, Duration};
use headless_chrome::Element;

use super::{betfair_tab::{BetfairTab, AsTab}, betfair_constants::{BETFAIR_CONSTANTS, BETFAIR_CSS_CONSTANTS}};
use crate::tabs::events_tab::{EventsTab, AsEventsTab, ContestantOdds, EventsTabError};

impl AsEventsTab for BetfairTab {
  fn get_events_tab(&self) -> &EventsTab {
    &self.events_tab
  }

  fn scrape_event(&self) -> Result<Vec<ContestantOdds>, EventsTabError> {
  // fn scrape_event(&self, venue_name: &str, event_time: DateTime<FixedOffset>) -> Result<Vec<ContestantOdds>, EventsTabError> {
    let mut contestant_odds = vec![];

    self.loop_schedule_items(&mut |a, b, c, d, e| {
      println!("{:?}\n{:?}", d.get_inner_text(), e.get_inner_text());
      println!("+++++++++++++");
    });
    // let target_venue_schedule_str = target_venue_schedule
    //   .ok_or(EventsTabError::BadScrape)?
    //   .get_inner_text()
    //   .map_err(|_| EventsTabError::BadScrape)?;
      // target_venue_schedule.get_inner_text() 
    
    // if let Ok() = target_venue_schedule.get_inner_text() {

    // }

    // for event in target_venue_schedule {
    //   if let Ok() = event.get_inner_text() {

    //   }
    // }
    
    Ok(contestant_odds)
  }
}
