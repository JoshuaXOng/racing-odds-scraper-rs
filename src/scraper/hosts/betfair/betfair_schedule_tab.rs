use std::{sync::Arc, collections::HashMap, hash::Hash};

use chrono::{Timelike, Duration, DateTime, FixedOffset};
use headless_chrome::{Element, protocol::cdp::DOM::Node, Tab as TabEngine};

use super::{betfair_tab::{BetfairTab, AsTab, Tab, AsBetfairTab}, betfair_constants::{BETFAIR_CONSTANTS, BETFAIR_CSS_CONSTANTS}};
use crate::{tabs::schedule_tab::{ScheduleTab, Event, AsScheduleTab, ScheduleTabError}, extensions::{rhc_extensions::{create_element_from_bnid, for_each_node}, datetime_extensions::change_datetimes_hhmm, str_extension::StrExtension}};

pub struct BetfairScheduleTab {
  pub betfair_tab: BetfairTab,
  pub schedule_tab: ScheduleTab,
  event_links: HashMap<String, Vec<EventLink>>
}

impl AsBetfairTab for BetfairScheduleTab {
  fn get_betfair_tab(&self) -> &BetfairTab {
    &self.betfair_tab
  }
}

impl AsTab for BetfairScheduleTab {
  fn get_tab(&self) -> &Tab {
    &self.betfair_tab.get_tab()
  }
}

impl AsScheduleTab for BetfairScheduleTab {
  fn get_schedule_tab(&self) -> &ScheduleTab {
    &self.schedule_tab
  }

  fn scrape_schedule(&self) -> Result<Vec<Event>, ScheduleTabError> {
    let mut event_details = vec![] as Vec<Event>;

    self.loop_schedule_items(&mut |captured_datetime, info_set| {
      let (venue_name_text, day_selected_text, venue_event_text) = if let (
        Ok(venue_name_text), 
        Ok(day_selected_text), 
        Ok(venue_event_text)
      ) = (
        info_set.venue_name.get_inner_text(), 
        info_set.day_selected.get_inner_text(), 
        info_set.venue_event.get_inner_text()
      ) {
        (venue_name_text, day_selected_text, venue_event_text)
      } else {
        return
      };

      let event_time = venue_event_text.split(":").collect::<Vec<_>>();

      let (event_time_hour, event_time_min) = if let (Some(Ok(event_time_hour)), Some(Ok(event_time_min))) = (
        event_time
          .get(0)
          .map(|event_time_hour| event_time_hour.parse::<u32>()), 
        event_time
          .get(1)
          .map(|event_time_min| event_time_min.parse::<u32>())
      ) {
        (event_time_hour, event_time_min)
      } else { 
        return
      };

      let event_start_datetime = if let Some(event_start_datetime) = captured_datetime
        .with_hour(event_time_hour)
        .and_then(|event_start_datetime| event_start_datetime.with_minute(event_time_min)) 
      {
        let days_schedule_ahead = match day_selected_text.as_str() {
          "Today" => 0,
          "Tomorrow" => 1, 
          _ => return
        };
        event_start_datetime + Duration::days(days_schedule_ahead as i64)
      } else {
        return
      };
      
      event_details.push(Event {
        venue_name: venue_name_text,
        planned_start_time: event_start_datetime,
        has_started: true,
      })
    }).or(Err(ScheduleTabError::BadScrape))?;
    
    Ok(event_details)
  }
}

impl BetfairScheduleTab {
  pub fn new(tab_engine: Arc<TabEngine>) -> Self {
    Self {
      betfair_tab: BetfairTab::new(tab_engine.clone()),
      schedule_tab: ScheduleTab::new(tab_engine.clone()),
      event_links: HashMap::from([])
    }
  }

  pub fn get_event_links(&self) -> Result<HashMap<String, Vec<EventLink>>, ()> {
    let mut event_links = HashMap::from([]);

    self.loop_schedule_items(&mut |datetime_of_scrape, schedule_entry| {      
      let venue_name_text = if let Ok(venue_name_text) = schedule_entry.venue_name.get_inner_text() {
        venue_name_text
      } else {
        return
      };
      
      let venue_event_text = if let Ok(venue_event_text) = schedule_entry.venue_event.get_inner_text() {
        venue_event_text
      } else {
        return
      };

      let split_vname_text = venue_event_text.split(":").collect::<Vec<_>>();
      let (&event_hh, &event_mm) = if let (Some(event_hh), Some(event_mm)) = (split_vname_text.get(0), split_vname_text.get(1)) {
        (event_hh, event_mm)
      } else {
        return
      };
      
      let days_schedule_ahead = match schedule_entry.day_selected.get_inner_text() {
        Ok(sday_text) if sday_text == String::from("Today") => 0, 
        Ok(sday_text) if sday_text == String::from("Tomorrow") => 1, 
        _ => return
      };

      let event_datetime = if let Ok(event_datetime) = change_datetimes_hhmm(
        datetime_of_scrape, 
        StrExtension(event_hh), 
        StrExtension(event_mm)
      ) {
        event_datetime + Duration::days(days_schedule_ahead as i64)
      } else {
        return
      };

      let event_href = if let Ok(Some(event_attributes)) = schedule_entry.venue_event.get_attributes() {
        let href_key_index = event_attributes.iter().position(|attribute| attribute == "href");
        let href_value_index = href_key_index.map(|index| index + 1);
        if let Some(href_value) = href_value_index.and_then(|index| event_attributes.get(index)) 
        { href_value.to_owned() } else { return }
      } else { return };

      let current_event_link = EventLink { 
        venue_name: venue_name_text.clone(),
        event_datetime,
        nav_link: String::from(BETFAIR_CONSTANTS.base_url) + &event_href,
      };
      event_links
        .entry(venue_name_text.clone())
        .or_insert(vec![current_event_link.clone()])
        .push(current_event_link.clone())
    })?;

    Ok(event_links)
  }

  fn loop_schedule_items(&self, callback: &mut dyn FnMut(DateTime<FixedOffset>, BetfairScheduleInfoSet) -> ()) -> Result<(), ()> {
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
          for_each_node(
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
            create_element_from_bnid(
              self
                .get_tab()
                .tab_engine
                .as_ref(), 
              venue_name.backend_node_id
            )?
          } else { continue };
          
          for venue_event in venue_events {
            let venue_event = create_element_from_bnid(
              self
                .get_tab()
                .tab_engine
                .as_ref(), 
              venue_event.backend_node_id
            )?;

            callback(browser_datetime, BetfairScheduleInfoSet {
              day_selected: interating_day, 
              schedule_tab, 
              venue_name: &venue_name, 
              venue_event: &venue_event
            });
          }
        }
      }
    };

    Ok(())
  }
}

//
// Miscellaneous Items.
//

#[derive(Debug, Clone)]
pub struct EventLink {
  pub venue_name: String,
  pub event_datetime: DateTime<FixedOffset>,
  pub nav_link: String
}

impl EventLink {
  pub fn run_uri_checks(&self) -> bool {
    self.nav_link.contains("betfair")
  }
}

pub struct BetfairScheduleInfoSet<'a> {
  pub day_selected: &'a Element<'a>,
  pub schedule_tab: &'a Element<'a>,
  pub venue_name: &'a Element<'a>,
  pub venue_event: &'a Element<'a>,
}

//
// End Miscellaneous Items.
//
