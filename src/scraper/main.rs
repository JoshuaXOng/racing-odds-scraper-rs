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
  let handler = thread::spawn(|| {
    let mut main_browser = Browser::new().unwrap();
    main_browser.open_page((TabType::Events, Host::Betfair)).unwrap();
  
    if let Some(events_tabs) = main_browser.events_tabs.get(&Host::Betfair) {
      if let Ok(odds) = events_tabs.scrape_event() {
        println!("{:#?}", &odds);
      }
    }
  });

  let handler2 = thread::spawn(|| {
    let mut main_browser = Browser::new().unwrap();
    main_browser.open_page((TabType::Events, Host::Betfair)).unwrap();
  
    if let Some(events_tabs) = main_browser.events_tabs.get(&Host::Betfair) {
      if let Ok(odds) = events_tabs.scrape_event() {
        println!("{:#?}", &odds);
      }
    }
  });

  handler.join().unwrap();
  handler2.join().unwrap();
  
  Ok(())
}