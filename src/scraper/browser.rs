use std::sync::Arc;

use headless_chrome::browser::Browser as BrowserEngine;
use crate::tabs::tab::Tab;
use crate::extensions::browser_engine::Extended;

pub struct Browser {
  pub browser_engine: BrowserEngine,
  pub browser_tabs: Vec<Tab>,
}

impl Browser {
  pub fn new() -> Result<Self, BrowserError> {
    Ok(Self {
      browser_engine: BrowserEngine::default().map_err(|_| BrowserError::OpenBrowser)?,
      browser_tabs: vec![],
    })
  }

  fn close_pages(&self, urls_to_close: Vec<&str>, exempted_urls: Vec<&str>) -> Result<(), BrowserError> {
    let mut has_encountered_error = false;

    let current_pages = self.browser_engine.get_tabs().try_lock().map_err(|_| BrowserError::ClosePage)?;
    for existing_page in current_pages.iter() {
      if !exempted_urls.contains(&existing_page.get_url().as_str()) && urls_to_close.contains(&existing_page.get_url().as_str()) {
        has_encountered_error = existing_page.close(false).is_ok();
      }
    }      
  
    if has_encountered_error { Ok(()) } else { Err(BrowserError::OpenPage) }  
  }

  fn open_pages(&mut self, urls_to_open: Vec<String>) -> Result<(), BrowserError> {
    let mut has_encountered_error = false;

    for url_to_open in  urls_to_open.iter() {
      let new_tab = self.browser_engine.new_tab().map_err(|_| BrowserError::OpenPage);
      if new_tab.is_err() {
        has_encountered_error = true;
        continue;
      }

      let tab_engine = new_tab.as_ref().unwrap().navigate_to(url_to_open);
      if tab_engine.is_err() {
        has_encountered_error = true;
        continue;
      }
      
      self.browser_tabs.push(Tab {
        tab_engine: new_tab.unwrap().clone()
      });
    };

    if has_encountered_error { Ok(()) } else { Err(BrowserError::OpenPage) }
  }

  fn clone(&self) -> Result<Self, BrowserError> {
    let mut new_browser = Browser::new()?;
    
    let from_urls = new_browser.browser_engine.get_current_urls()
      .map_err(|_| BrowserError::OpenBrowser)?;

    new_browser.open_pages(from_urls)?;

    Ok(new_browser)
  }
}

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
