use chrono::NaiveDateTime;

use super::{betfair_tab::{BetfairTab, AsTab}, betfair_constants::{BETFAIR_CONSTANTS, BETFAIR_CSS_CONSTANTS}};
use crate::{tabs::schedule_tab::{ScheduleTab, Event, AsScheduleTab, ScheduleTabError}, extensions::vec_extension::VecExtension};

impl AsScheduleTab for BetfairTab {
  fn get_schedule_tab(&self) -> &ScheduleTab {
    &self.schedule_tab
  }

  fn scrape_schedule(&self) -> Result<Vec<Event>, ScheduleTabError> {
    self.goto_url(BETFAIR_CONSTANTS.racing_url)
      .map_err(|_| ScheduleTabError::BadScrape)?;

    let schedule_tabs = self.get_tab().tab_engine.wait_for_elements(
      format!(".{}", BETFAIR_CSS_CONSTANTS.schedule_tab_class).as_str()
    )
      .map_err(|_| ScheduleTabError::BadScrape)?;
    
    let mut event_details: Vec<Event> = vec![];

    schedule_tabs.iter().try_for_each(|schedule_tab| -> Result<(), ScheduleTabError> {
      schedule_tab.click()
        .map_err(|_| ScheduleTabError::BadScrape)?;
      
      let venue_schedules = self.get_tab().tab_engine.wait_for_elements(
        format!(".{}", BETFAIR_CSS_CONSTANTS.venue_schedule_class).as_str()
      )
        .map_err(|_| ScheduleTabError::BadScrape)?;
        
      venue_schedules.iter().for_each(|venue_schedule| {
        if let Ok(venue_schedule) = venue_schedule.get_inner_text() {
          let venue_schedule_parts = venue_schedule.split("\n").collect::<Vec<&str>>();
          
          let venue_name = venue_schedule_parts.get(0);
          let event_times = &venue_schedule_parts[1..];

          if let Some(venue_name) = venue_name {
            event_times.iter().for_each(|event_time| {
              event_details.push(Event {
                venue_name: String::from(*venue_name),
                planned_start_time: NaiveDateTime::from_timestamp(100, 10),
                has_started: true,
              })
            })
          }
        }
      });

      Ok(())
    })?;

    dbg!(self.get_datetime());

    Ok(vec![Event {
      venue_name: String::from("sd"),
      planned_start_time: NaiveDateTime::from_timestamp(100, 10),
      has_started: true,
    }])
  }
}
