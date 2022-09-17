use chrono::{Timelike, Duration};
use headless_chrome::Element;

use super::{betfair_tab::{BetfairTab, AsTab}, betfair_constants::{BETFAIR_CONSTANTS, BETFAIR_CSS_CONSTANTS}};
use crate::{tabs::schedule_tab::{ScheduleTab, Event, AsScheduleTab, ScheduleTabError}};

impl AsScheduleTab for BetfairTab {
  fn get_schedule_tab(&self) -> &ScheduleTab {
    &self.schedule_tab
  }

  fn scrape_schedule(&self) -> Result<Vec<Event>, ScheduleTabError> {
    let mut event_details: Vec<Event> = vec![];

    self.goto_url(BETFAIR_CONSTANTS.racing_url)
      .map_err(|_| ScheduleTabError::BadScrape)?;

    let browser_datetime = self.get_datetime()
      .map_err(|_| ScheduleTabError::BadScrape)?;

    let days_available = self.get_tab()
      .tab_engine
      .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.schedule_day_class).as_str())
      .map_err(|_| ScheduleTabError::BadScrape)?;
    
    let select_first_day = days_available.get(0)
      .map(|first_day| 
        first_day.get_inner_text().map(|fd_text| fd_text == "Today").unwrap_or(false) && 
        first_day.click().is_ok())
      .unwrap_or(false);
    if !select_first_day { Err(ScheduleTabError::BadScrape)? }
      
    let mut iterating_days = vec![] as Vec<&Element>;
    if let Some(first_day) = days_available.get(0) { iterating_days.push(first_day); }
    days_available.get(1)
      .map(|tomorrow| {
        let tomorrow_str = tomorrow.get_inner_text()
          .unwrap_or(String::from(""));
        if tomorrow_str == "Tomorrow" {
          iterating_days.push(tomorrow)
        }
      });

    for (iday_index, &interating_day) in iterating_days.iter().enumerate() {
      if interating_day.click().is_err() { continue }

      let schedule_tabs = if let Ok(schedule_tabs) = self.get_tab()
        .tab_engine
        .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.schedule_tab_class).as_str())
      { schedule_tabs } else { continue };

      for schedule_tab in schedule_tabs {
        if let Ok(_) = schedule_tab.click() {
        } else { continue };

        let venue_schedules = if let Ok(venue_schedules) =
          self.get_tab()
            .tab_engine
            .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.venue_schedule_class).as_str()) 
        { venue_schedules } else { continue };
        
        for venue_schedule in &venue_schedules {
          let venue_schedule_str = if let Ok(venue_schedule_str) = venue_schedule.get_inner_text()
          { venue_schedule_str } else { continue };
          
          let venue_schedule_parts = venue_schedule_str.split("\n")
            .collect::<Vec<&str>>();
      
          let venue_name = if let Some(venue_name) = venue_schedule_parts.get(0)
          { venue_name } else { continue };
          let desc_or_event_time = venue_schedule_parts.get(1)
            .map(|desc_or_event_time| desc_or_event_time.parse::<u32>());
  
          let event_times = match desc_or_event_time {
            Some(Ok(_)) => &venue_schedule_parts[1..],
            Some(Err(_)) => &venue_schedule_parts[2..],
            None => &([] as [&str; 0])[..]
          };
  
          for event_time in event_times {
            let hour_and_min = event_time.split(":").collect::<Vec<&str>>();
            let hour = if let Some(Ok(hour)) = hour_and_min.get(0)
              .map(|hour| hour.parse::<u32>())
            { hour } else { continue };
            let min = if let Some(Ok(min)) = hour_and_min.get(1)
              .map(|min| min.parse::<u32>())
            { min } else { continue };
            
            let event_datetime = if let Some(event_datetime) = browser_datetime.clone()
              .with_hour(hour)
              .and_then(|browser_datetime| browser_datetime.with_minute(min))
            { event_datetime } else { continue };
  
            event_details.push(Event {
              venue_name: String::from(*venue_name),
              planned_start_time: event_datetime + Duration::days(iday_index as i64),
              has_started: true,
            })
          }
        }
      }
    }

  
    Ok(event_details)
  }
}
