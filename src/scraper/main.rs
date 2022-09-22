use std::{error::Error, thread, sync::Arc};

use browser::{Browser, TabType, Host};
use chrono::DateTime;

use crate::extensions::vec_extension::VecExtension;

mod tabs; 
mod hosts; 
mod models;
mod orchestrators;
mod browser;
mod extensions;

fn main() -> Result<(), Box<dyn Error>> {
  let mut main_browser = Browser::new().unwrap();
  main_browser.open_page((TabType::Events, Host::Betfair)).unwrap();
  main_browser.open_page((TabType::Schedule, Host::Betfair)).unwrap();

  if let Some(events_tab) = main_browser.events_tabs.get(&Host::Betfair) {
    println!("{:?}", events_tab.scrape_event("sad", DateTime::parse_from_str("2022 Apr 13 12:09:14.274 +0000", "%Y %b %d %H:%M:%S%.3f %z").unwrap()).unwrap());
  }
  
  // if let Some(schedule_tab) = main_browser.schedule_tabs.get(&Host::Betfair) {
  //   println!("{:?}", schedule_tab.scrape_schedule().unwrap());
  // }
  
  Ok(())
}