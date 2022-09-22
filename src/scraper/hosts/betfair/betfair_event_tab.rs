use std::sync::Arc;

use chrono::{DateTime, FixedOffset, Duration};
use headless_chrome::{Element, Tab as TabEngine};

use super::{betfair_tab::{BetfairTab, AsTab, Tab, AsBetfairTab, ScheduleTab}, betfair_constants::{BETFAIR_CONSTANTS, BETFAIR_CSS_CONSTANTS}, betfair_schedule_tab::BetfairScheduleTab};
use crate::{tabs::{events_tab::{EventsTab, AsEventsTab, ContestantOdds, EventsTabError}, schedule_tab::AsScheduleTab}, extensions::rhc_extensions::{for_each_node, is_node_of_class, create_element_from_bnid}, models::{odds::Odds, money::Money}};

pub struct BetfairEventsTab {
  pub betfair_tab: BetfairTab,
  pub events_tab: EventsTab,
  schedule_tab: BetfairScheduleTab,
}

impl AsBetfairTab for BetfairEventsTab {
  fn get_betfair_tab(&self) -> &BetfairTab {
    &self.betfair_tab
  }
}

impl AsTab for BetfairEventsTab {
  fn get_tab(&self) -> &Tab {
    &self.betfair_tab.get_tab()
  }
}

impl AsEventsTab for BetfairEventsTab {
  fn get_events_tab(&self) -> &EventsTab {
    &self.events_tab
  }

  fn scrape_event(&self, venue_name: &str, event_time: DateTime<FixedOffset>) -> Result<Vec<ContestantOdds>, EventsTabError> {
    // let event_links = self
    //   .schedule_tab
    //   .get_event_links()
    //   .or(Err(EventsTabError::BadScrape))?;

    // let v_event_links = event_links
    //   .get(venue_name)
    //   .ok_or(EventsTabError::BadScrape)?;
    
    // let v_event_link = v_event_links.iter().find(|vevent_link| vevent_link.event_datetime == event_time)
    //   .ok_or(EventsTabError::BadScrape)?;

    // self.goto_url(v_event_link.nav_link.as_str())
    //   .or(Err(EventsTabError::BadScrape))?;
    
    self.goto_url("https://www.betfair.com.au/exchange/plus/horse-racing/market/1.203704450")
      .or(Err(EventsTabError::BadScrape))?;

    let mut contestant_odds = vec![];
    
    let contestant_entries = &self
      .get_tab()
      .tab_engine
      .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.contestant_entry_class).as_str())
      .or(Err(EventsTabError::BadScrape))?;
    
    for contestant_entry in contestant_entries {
      let c_entry_node = if let Ok(c_entry_node) = contestant_entry.get_description()
      { c_entry_node.to_owned() } else { continue };

      let mut contestant_name = None as Option<String>;
      let mut back_entries = vec![] as Vec<Odds>;
      let mut lay_entries = vec![] as Vec<Odds>;
      for_each_node(&c_entry_node, &mut |node| {
        if is_node_of_class(node, "runner-name") {
          contestant_name = create_element_from_bnid(&self.get_tab().tab_engine, node.backend_node_id)
            .and_then(|a| a.get_inner_text().or(Err(())))
            .ok()
            .map(|contestant_name| contestant_name);
        }

        let is_back_button = is_node_of_class(node, "back-button");
        let is_lay_button = is_node_of_class(node, "lay-button");
        if is_back_button || is_lay_button {
          let entry_text = if let Ok(entry_text) = create_element_from_bnid(&self.get_tab().tab_engine, node.backend_node_id)
            .and_then(|entry| entry.get_inner_text().or(Err(()))) 
          { entry_text } else { return };

          let odds_and_money = entry_text.split_whitespace().collect::<Vec<_>>();
          let back_odds = dbg!(odds_and_money.get(0));
          let back_money = dbg!(odds_and_money.get(1));

          if let (Some(&odds), Some(&money)) = (back_odds, back_money) {
            let entry_container = if is_back_button { &mut back_entries } else { &mut lay_entries };
            if let Ok(odds) = (odds, money).try_into() as Result<Odds, ()> {
              entry_container.push(odds);
            }
          }
        }
      });

      if let Some(contestant_name) = contestant_name {
        contestant_odds.push(ContestantOdds {
          contestant_name: contestant_name,
          backing_odds: back_entries,
          laying_odds: lay_entries
        })
      }
    }

    Ok(contestant_odds)
  }
}

impl BetfairEventsTab {
  pub fn new(tab_engine: Arc<TabEngine>) -> Self {
    Self {
      betfair_tab: BetfairTab::new(tab_engine.clone()),
      events_tab: EventsTab::new(tab_engine.clone()),
      schedule_tab: BetfairScheduleTab::new(tab_engine.clone())
    }
  }
}
