use std::sync::Arc;

use chrono::{DateTime, FixedOffset};
use headless_chrome::protocol::cdp::DOM::Node;
use headless_chrome::{Tab as TabEngine, Element};

pub use crate::tabs::tab::{Tab, AsTab};
pub use crate::tabs::schedule_tab::ScheduleTab;
pub use crate::tabs::events_tab::EventsTab;

use super::betfair_constants::{BETFAIR_CONSTANTS, BETFAIR_CSS_CONSTANTS};

pub struct BetfairTab {
  pub tab: Tab,
  pub schedule_tab: ScheduleTab,
  pub events_tab: EventsTab,
}

impl BetfairTab {
  pub fn new(tab_engine: Arc<TabEngine>) -> Self {
    Self {
      tab: Tab { tab_engine: tab_engine.clone() },
      schedule_tab: ScheduleTab { tab_engine: tab_engine.clone() },
      events_tab: EventsTab { tab_engine: tab_engine.clone() },
    }
  }    

  fn loop_schedule_items(&self, callback: &dyn Fn(DateTime<FixedOffset>, &Element, &Element, &Element, &Element) -> ()) -> Result<(), ()> {
    self.goto_url(BETFAIR_CONSTANTS.racing_url)
      .or(Err(()))?;

    let browser_datetime = self.get_datetime()
      .or(Err(()))?;

    let days_available = self.get_tab()
      .tab_engine
      .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.schedule_day_class).as_str())
      .or(Err(()))?;
    
    let select_first_day = days_available.get(0)
      .map(|first_day| 
        first_day.get_inner_text().map(|fd_text| fd_text == "Today").unwrap_or(false) && 
        first_day.click().is_ok())
      .unwrap_or(false);
    if !select_first_day { Err(())? }
      
    let mut iterating_days = vec![] as Vec<&Element>;
    if let Some(first_day) = days_available.get(0) { iterating_days.push(first_day); }
    days_available.get(1)
      .map(|tomorrow| {
        let tomorrow_text = tomorrow.get_inner_text()
          .unwrap_or(String::from(""));
        if tomorrow_text == "Tomorrow" {
          iterating_days.push(tomorrow)
        }
      });

    for interating_day in iterating_days {
      if interating_day.click().is_err() { continue }

      for schedule_tab in if let Ok(schedule_tabs) = 
        &self.get_tab()
          .tab_engine
          .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.schedule_tab_class).as_str())
        { schedule_tabs } else { continue }
      {
        if let Ok(_) = schedule_tab.click() {} 
        else { continue };

        for venue_schedule in if let Ok(venue_schedules) = 
          self.get_tab()
            .tab_engine
            .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.venue_schedule_class).as_str()) 
          { venue_schedules } else { continue }
        {
          let venue_schedule = if let Ok(venue_schedule) = venue_schedule.get_description() 
          { venue_schedule } else { continue };
          
          let mut venue_name = None as Option<Node>;
          let mut venue_events = vec![] as Vec<Node>;
          self.for_each_node(
            &venue_schedule,
            &mut |node: &Node| { 
              if 
                node.local_name == "div" &&
                node.attributes.as_ref()
                  .map(|attributes| attributes.contains(&String::from(BETFAIR_CSS_CONSTANTS.venue_name_class)))
                  .unwrap_or(false) 
              { 
                venue_name = Some(node.clone());
              }

              if 
                node.local_name == "a" &&
                node.attributes.as_ref()
                  .map(|attributes| attributes.contains(&String::from(BETFAIR_CSS_CONSTANTS.venue_event_class)))
                  .unwrap_or(false) 
              { 
                venue_events.push(node.clone());
              }
            }
          );

          let venue_name = if let Some(venue_name) = venue_name { 
            if let Ok(venue_name) = Element::new(self.get_tab().tab_engine.as_ref(), venue_name.node_id)
            { venue_name } else { continue }
          } else { continue };
          
          for venue_event in venue_events {
            let venue_event = if let Ok(venue_event) = Element::new(self.get_tab().tab_engine.as_ref(), venue_event.node_id)
            { venue_event } else { continue };

            callback(browser_datetime, interating_day, schedule_tab, &venue_name, &venue_event);
          }
        }
      }
    };

    Ok(())
  }
}

impl AsTab for BetfairTab {
  fn get_tab(&self) -> &Tab {
    &self.tab
  }
}
