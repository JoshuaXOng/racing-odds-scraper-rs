use chrono::Timelike;
use serde_json::de;

use super::{betfair_tab::{BetfairTab, AsTab}, betfair_constants::{BETFAIR_CONSTANTS, BETFAIR_CSS_CONSTANTS}};
use crate::{tabs::schedule_tab::{ScheduleTab, Event, AsScheduleTab, ScheduleTabError}};

impl AsScheduleTab for BetfairTab {
  fn get_schedule_tab(&self) -> &ScheduleTab {
    &self.schedule_tab
  }

  fn scrape_schedule(&self) -> Result<Vec<Event>, ScheduleTabError> {
    // self.goto_url(BETFAIR_CONSTANTS.racing_url)
    //   .map_err(|_| ScheduleTabError::BadScrape)?;

    // let schedule_tabs = self.get_tab().tab_engine
    //   .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.schedule_tab_class).as_str())
    //   .map_err(|_| ScheduleTabError::BadScrape)?;
    
    // let browser_datetime = self.get_datetime()
    //   .map_err(|_| ScheduleTabError::BadScrape)?;
      
    // let mut event_details: Vec<Event> = vec![];

    // schedule_tabs.iter().try_for_each(|schedule_tab| -> Result<(), ScheduleTabError> {
    //   schedule_tab.click().map_err(|_| ScheduleTabError::BadScrape)?;
      
    //   let venue_schedules = self.get_tab().tab_engine.wait_for_elements(
    //     format!(".{}", BETFAIR_CSS_CONSTANTS.venue_schedule_class).as_str()
    //   ).map_err(|_| ScheduleTabError::BadScrape)?;
        
    //   venue_schedules.iter().for_each(|venue_schedule| {
    //     if let Ok(venue_schedule) = venue_schedule.get_inner_text() {
    //       let venue_schedule_parts = venue_schedule.split("\n").collect::<Vec<&str>>();
          
    //       let venue_name = venue_schedule_parts.get(0);
    //       let event_times = &venue_schedule_parts[1..];
          
    //       if let Some(venue_name) = venue_name {
    //         event_times.iter().for_each(|event_time| {
    //           let hour_and_min = event_time.split(":").collect::<Vec<&str>>();
    //           let hour = hour_and_min.get(0).map(|hour| hour.parse::<u32>());
    //           let min = hour_and_min.get(1).map(|min| min.parse::<u32>());
    //           if let (Some(Ok(hour)), Some(Ok(min))) = (hour, min) {
    //             if let Some(browser_datetime) = browser_datetime.clone().with_hour(hour).and_then(|browser_datetime| browser_datetime.with_minute(min)) {
    //               event_details.push(Event {
    //                 venue_name: String::from(*venue_name),
    //                 planned_start_time: browser_datetime,
    //                 has_started: true,
    //               })
    //             }
    //           }
    //         })
    //       }
    //     }
    //   });

    //   Ok(())
    let mut event_details: Vec<Event> = vec![];

    let nav_result = self.goto_url(BETFAIR_CONSTANTS.racing_url);
    let schedule_tabs = self.get_tab().tab_engine
      .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.schedule_tab_class).as_str());
    let browser_datetime = self.get_datetime();
    
    if (&nav_result).is_err() || (&schedule_tabs).is_err() || (&browser_datetime).is_err() {
      return Err(ScheduleTabError::BadScrape);
    }

    for schedule_tab in schedule_tabs.unwrap() {
      let selected_tab = schedule_tab.click();
      let venue_schedules = self.get_tab().tab_engine
        .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.venue_schedule_class).as_str());
      
      if (&selected_tab).is_err() || (&venue_schedules).is_err() {
        continue;
      }

      for venue_schedule in &venue_schedules.unwrap() {
        let venue_schedule_str = venue_schedule.get_inner_text();
        if venue_schedule_str.is_err() {
          continue;
        }

        let venue_schedule_parts = venue_schedule_str.as_ref().unwrap().split("\n").collect::<Vec<&str>>();

        let venue_name = venue_schedule_parts.get(0);
        let desc_or_event_time = venue_schedule_parts.get(1)
          .map(|desc_or_event_time| desc_or_event_time.parse::<u32>());

        let event_times = match (venue_name, desc_or_event_time) {
          (Some(_), Some(Ok(_))) => &venue_schedule_parts[1..],
          (Some(_), Some(Err(_))) => &venue_schedule_parts[2..],
          (Some(_), None) => &([] as [&str; 0])[..],
          _ => continue,
        };

        for event_time in event_times {
          let hour_and_min = event_time.split(":").collect::<Vec<&str>>();
          let hour = hour_and_min.get(0).map(|hour| hour.parse::<u32>());
          let min = hour_and_min.get(1).map(|min| min.parse::<u32>());
          
          if let ((&None, &None), (&Some(Err(_)), &Some(Err(_)))) = ((&hour, &min), (&hour, &min)) {
            continue;
          }
          
          let event_datetime = browser_datetime.as_ref().unwrap().clone()
            .with_hour(hour.unwrap().unwrap())
            .and_then(|browser_datetime| browser_datetime.with_minute(min.unwrap().unwrap()));
          if let None = event_datetime {
            continue;
          }

          event_details.push(Event {
            venue_name: String::from(*venue_name.unwrap()),
            planned_start_time: event_datetime.unwrap(),
            has_started: true,
          })
        }
      }
    }

    Ok(event_details)
  }
}
