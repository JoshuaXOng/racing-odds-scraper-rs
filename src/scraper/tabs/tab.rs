use std::fs;
use std::sync::Arc;
use std::path::Path;

use rand::Rng;
use chrono::{DateTime, FixedOffset};
use headless_chrome::browser::tab::Tab as TabEngine;
use headless_chrome::browser::tab::point::Point;
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;

pub struct Tab {
  pub tab_engine: Arc<TabEngine>,
}

pub trait AsTab {
  fn get_tab(&self) -> &Tab;

  fn goto_url(&self, target_url: &str) -> Result<(), TabError> {
    self.get_tab().tab_engine.navigate_to(target_url)
      .map_err(|_| TabError::Navigate)?;
    
    Ok(())
  }

  fn refresh_page(&self) -> Result<(), TabError> {
    self.get_tab().tab_engine.reload(false, None)
      .map_err(|_| TabError::Reload)?;

    Ok(())
  }

  fn take_screenshot(&self, save_to_path: &Path) -> Result<(), TabError> {
    let capture_data = self.get_tab().tab_engine.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, false)
      .map_err(|_| TabError::Screenshot)?;
    
    fs::write(save_to_path, &capture_data)
      .map_err(|_| TabError::Screenshot)?;

    Ok(())
  }

  fn get_datetime(&self) -> Result<DateTime<FixedOffset>, TabError> {
    let js_datetime = self.get_tab().tab_engine.evaluate("(new Date()).toUTCString()", false)
      .map_err(|_| TabError::Evaluate)?;
    
    Ok(DateTime::parse_from_rfc2822(
      js_datetime.value.ok_or(TabError::Evaluate)?.as_str().ok_or(TabError::Evaluate)?
    ).map_err(|_| TabError::Evaluate)?)
  }

  fn fake_mouse_movement(&self) -> Result<(), TabError> {
    let tab_bounds = self.get_tab().tab_engine.get_bounds()
      .map_err(|_| TabError::Action)?;

    self.get_tab().tab_engine.move_mouse_to_point(Point { 
      x: rand::thread_rng().gen_range(0..=tab_bounds.width as i32) as f64, 
      y: rand::thread_rng().gen_range(0..=tab_bounds.height as i32) as f64
    });

    Ok(())
  }
}

//
// Tab Errors.
//

#[derive(Debug)]
pub enum TabError {
  Screenshot,
  Reload,
  Evaluate,
  Navigate,
  Action,
}

impl std::fmt::Display for TabError {
  fn fmt(&self, out_formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      TabError::Screenshot => write!(out_formatter, "Failed to take screenshot."),
      TabError::Reload => write!(out_formatter, "Failed to reload tab."),
      TabError::Evaluate => write!(out_formatter, "Failed to evaluate JS expression."),
      TabError::Navigate => write!(out_formatter, "Failed to navigate in tab."),
      TabError::Action => write!(out_formatter, "Failed to execute on action."),
    }
  }
}

impl std::error::Error for TabError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      TabError::Screenshot => None,
      TabError::Reload => None,
      TabError::Evaluate => None,
      TabError::Navigate => None,
      TabError::Action => None,
    }
  }
}

//
// End Tab Errors.
//
