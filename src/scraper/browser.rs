use std::collections::HashMap;
use std::hash::Hash;

use crate::hosts::betfair::betfair_event_tab::BetfairEventsTab;
use crate::hosts::betfair::betfair_schedule_tab::BetfairScheduleTab;
use crate::tabs::events_tab::AsEventsTab;
use crate::tabs::schedule_tab::AsScheduleTab;
use headless_chrome::browser::Browser as BrowserEngine;
use headless_chrome::LaunchOptionsBuilder;

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
            .or(Err(BrowserError::General))?;

        let browser_engine = BrowserEngine::new(browser_options).or(Err(BrowserError::General))?;

        browser_engine
            .wait_for_initial_tab()
            .or(Err(BrowserError::General))?;

        Ok(Self {
            browser_engine,
            events_tabs: HashMap::from([]),
            schedule_tabs: HashMap::from([]),
        })
    }

    pub fn open_page(&mut self, (tab_type, host): (TabType, Host)) -> Result<(), BrowserError> {
        let tab_engine = if let Ok(tab_engine) = self.browser_engine.new_tab() {
            tab_engine
        } else {
            Err(BrowserError::General)?
        };

        match tab_type {
            TabType::Events => {
                let tab = self.events_tabs.get(&host.clone());
                if tab.is_some() {
                    return Ok(());
                };

                match host.clone() {
                    Host::Betfair => self
                        .events_tabs
                        .insert(host.clone(), Box::new(BetfairEventsTab::new(tab_engine))),
                };
            }
            TabType::Schedule => {
                let tab = self.schedule_tabs.get(&host.clone());
                if tab.is_some() {
                    return Ok(());
                };

                match host.clone() {
                    Host::Betfair => self
                        .schedule_tabs
                        .insert(host.clone(), Box::new(BetfairScheduleTab::new(tab_engine))),
                };
            }
        };

        Ok(())
    }

    #[allow(dead_code)]
    pub fn close_page(&self, (tab_type, host_name): (TabType, Host)) -> Result<(), BrowserError> {
        match (tab_type, host_name.clone()) {
            (TabType::Events, _) => drop(
                self.events_tabs
                    .get(&host_name.clone())
                    .ok_or(BrowserError::General)?,
            ),
            (TabType::Schedule, _) => drop(
                self.schedule_tabs
                    .get(&host_name.clone())
                    .ok_or(BrowserError::General)?,
            ),
        };

        Ok(())
    }
}

//
// Misc Types.
//

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Host {
    Betfair,
}

#[allow(dead_code)]
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
    General,
}

impl std::fmt::Display for BrowserError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            BrowserError::General => write!(formatter, "Browser operation failed."),
        }
    }
}

impl std::error::Error for BrowserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            BrowserError::General => None,
        }
    }
}

impl From<String> for BrowserError {
    fn from(_: String) -> Self {
        BrowserError::General
    }
}

//
// End Browser Errors.
//
