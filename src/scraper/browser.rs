use std::collections::HashMap;
use std::hash::Hash;

use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::browser::Browser as BrowserEngine;
use crate::hosts::betfair::betfair_event_tab::BetfairEventsTab;
use crate::hosts::betfair::betfair_schedule_tab::BetfairScheduleTab;
use crate::hosts::betfair::betfair_tab::BetfairTab;
use crate::tabs::events_tab::AsEventsTab;
use crate::tabs::schedule_tab::AsScheduleTab;

pub struct Browser {
  pub browser_engine: BrowserEngine,
  pub events_tabs: HashMap<Host, Box<dyn AsEventsTab>>,
  pub schedule_tabs: HashMap<Host, Box<dyn AsScheduleTab>>,
}

impl Browser {
  pub fn new() -> Result<Self, BrowserError> {
    let browser_options = LaunchOptionsBuilder::default()
      .headless(false)
      .build()
      .map_err(|_| BrowserError::OpenBrowser)?;

    let browser_engine = BrowserEngine::new(browser_options)
      .map_err(|_| BrowserError::OpenBrowser)?;
    browser_engine.wait_for_initial_tab()
      .map_err(|_| BrowserError::OpenBrowser)?;
    
    Ok(Self {
      browser_engine,
      events_tabs: HashMap::from([]),
      schedule_tabs: HashMap::from([]),
    })
  }

  pub fn open_page(&mut self, (tab_type, host_name): (TabType, Host)) -> Result<(), BrowserError> {
    match (tab_type, host_name.clone()) {
      (TabType::Events, _) => {
        let tab = self.events_tabs.get(&host_name.clone());
        if tab.is_some() { 
          return Ok(()); 
        };
        
        match (self.browser_engine.new_tab(), host_name.clone()) {
          (Err(_), _) => Err(BrowserError::OpenPage)?,
          (Ok(tab_engine), Host::Betfair) => self.events_tabs
            .insert(host_name.clone(), Box::new(BetfairEventsTab::new(tab_engine)))
        };
      },
      (TabType::Schedule, _) => {
        let tab = self.schedule_tabs.get(&host_name.clone());
        if tab.is_some() { 
          return Ok(()); 
        };
        
        match (self.browser_engine.new_tab(), host_name.clone()) {
        (Err(_), _) => Err(BrowserError::OpenPage)?,
        (Ok(tab_engine), Host::Betfair) => self.schedule_tabs
          .insert(host_name.clone(), Box::new(BetfairScheduleTab::new(tab_engine)))
        };
      }
    };

    Ok(())
  }

  pub fn close_page(&self, (tab_type, host_name): (TabType, Host)) -> Result<(), BrowserError> {
    match (tab_type, host_name.clone()) {
      (TabType::Events, _) => {
        drop(
          self.events_tabs.get(&host_name.clone())
            .ok_or(BrowserError::ClosePage)?
        )
      },
      (TabType::Schedule, _) => {
        drop(
          self.schedule_tabs.get(&host_name.clone())
            .ok_or(BrowserError::ClosePage)?
        )
      }
    };

    Ok(())
  }
}

//
// Misc Types.
//

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Host {
  Betfair
}

pub enum TabType {
  Events,
  Schedule,
}

//
// End Misc Types.
//

//
// Browser Errors.
//

#[derive(Debug)]
pub enum BrowserError {
  OpenBrowser,
  ReadInfo,
  OpenPage,
  ClosePage,
}

impl std::fmt::Display for BrowserError {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      BrowserError::OpenBrowser => write!(formatter, "Failed to create new browser instance."),
      BrowserError::ReadInfo => write!(formatter, "Failed to read information form browser"),
      BrowserError::OpenPage => write!(formatter, "Failed to open a new page instance."),
      BrowserError::ClosePage => write!(formatter, "Failed to close a new page instance."),
    }
  }
}

impl std::error::Error for BrowserError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      BrowserError::OpenBrowser => None,
      BrowserError::ReadInfo => None,
      BrowserError::OpenPage => None,
      BrowserError::ClosePage => None,
    }
  }
}

impl From<String> for BrowserError {
  fn from(_: String) -> Self {
    BrowserError::OpenBrowser
  }
}

//
// End Browser Errors.
//
