use chrono::{DateTime, FixedOffset, Duration};
use headless_chrome::Element;

use super::{betfair_tab::{BetfairTab, AsTab}, betfair_constants::{BETFAIR_CONSTANTS, BETFAIR_CSS_CONSTANTS}};
use crate::tabs::events_tab::{EventsTab, AsEventsTab, ContestantOdds, EventsTabError};

impl AsEventsTab for BetfairTab {
  fn get_events_tab(&self) -> &EventsTab {
    &self.events_tab
  }

  fn scrape_event(&self, venue_name: &str, event_time: DateTime<FixedOffset>) -> Result<Vec<ContestantOdds>, EventsTabError> {
    let mut contestant_odds = vec![];

    self.goto_url(BETFAIR_CONSTANTS.racing_url)
      .map_err(|_| EventsTabError::BadScrape)?;

    let browser_datetime = self.get_datetime()
      .map_err(|_| EventsTabError::BadScrape)?;

    let days_available = self.get_tab()
      .tab_engine
      .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.schedule_day_class).as_str())
      .map_err(|_| EventsTabError::BadScrape)?;

    let select_first_day = days_available.get(0)
      .map(|first_day| 
        first_day.get_inner_text().map(|fd_text| fd_text == "Today").unwrap_or(false) && 
        first_day.click().is_ok())
      .unwrap_or(false);
    if !select_first_day { Err(EventsTabError::BadScrape)? }
      
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

    let mut target_venue_schedule = None as Option<Element>;

    'outer: for (iday_index, &interating_day) in iterating_days.iter().enumerate() {
      if interating_day.click().is_err() { continue }
      
      if (browser_datetime.clone() + Duration::days(iday_index as i64)).date_naive() != 
        event_time.clone().date_naive() 
      { continue };

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

        for venue_schedule in venue_schedules {
          let venue_name_ = if let Ok(Some(venue_name)) = venue_schedule.get_inner_text()
            .map(|venue_schedule_str| 
              venue_schedule_str
                .split("\n")
                .collect::<Vec<&str>>()
                .get(0)
                .map(|venue_name| String::from(*venue_name))
            )
          { venue_name } else { continue };
          
          if venue_name_ == venue_name {
            target_venue_schedule = Some(venue_schedule);
            break 'outer;
          }
        }
      }
    }
    // target_venue_schedule.unwrap().
    let tvenue_schedule_str = target_venue_schedule
      .ok_or(EventsTabError::BadScrape)?
      .get_inner_text()
      .map_err(|_| EventsTabError::BadScrape)?;
    
    let tvenue_schedule_parts = tvenue_schedule_str.split("\n")
      .collect::<Vec<&str>>();

    let desc_or_event_time = tvenue_schedule_parts.get(1)
      .map(|desc_or_event_time| desc_or_event_time.parse::<u32>());

    let event_times = match desc_or_event_time {
      Some(Ok(_)) => &tvenue_schedule_parts[1..],
      Some(Err(_)) => &tvenue_schedule_parts[2..],
      None => &([] as [&str; 0])[..]
    };

    for even_time in event_times {

    }
    
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
