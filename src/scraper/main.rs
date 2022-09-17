use core::time;
use std::{error::Error, thread::sleep};

use browser::{Browser, TabType, Host};

use crate::extensions::vec_extension::VecExtension;

mod tabs; 
mod hosts; 
mod models;
mod orchestrators;
mod browser;
mod extensions;

fn main() -> Result<(), Box<dyn Error>> {
  let mut main_browser = Browser::new()?;
  main_browser.open_page((TabType::Schedule, Host::Betfair))?;

  if let Some(schedule_tab) = main_browser.schedule_tabs.get(&Host::Betfair) {
    if let Ok(schedule) = schedule_tab.scrape_schedule() {
      println!("{}", VecExtension(&schedule));
    }
  }
  
  Ok(())
}