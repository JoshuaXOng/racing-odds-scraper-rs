use std::fs;
use std::path::Path;
use std::sync::Arc;

use chrono::{DateTime, FixedOffset};
use headless_chrome::browser::tab::Tab as TabEngine;
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;

pub struct Tab {
    pub tab_engine: Arc<TabEngine>,
}

pub trait AsTab {
    fn get_tab(&self) -> &Tab;

    fn goto_url(&self, target_url: &str) -> Result<(), TabError> {
        self.get_tab()
            .tab_engine
            .navigate_to(target_url)
            .or(Err(TabError::General))?;

        Ok(())
    }

    fn refresh_page(&self) -> Result<(), TabError> {
        self.get_tab()
            .tab_engine
            .reload(false, None)
            .or(Err(TabError::General))?;

        Ok(())
    }

    fn take_screenshot(&self, save_to_path: &Path) -> Result<(), TabError> {
        let capture_data = self
            .get_tab()
            .tab_engine
            .capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, false)
            .or(Err(TabError::General))?;

        fs::write(save_to_path, &capture_data).or(Err(TabError::General))?;

        Ok(())
    }

    fn get_datetime(&self) -> Result<DateTime<FixedOffset>, TabError> {
        let js_datetime = self
            .get_tab()
            .tab_engine
            .evaluate("(new Date()).toUTCString()", false)
            .or(Err(TabError::General))?;

        Ok(DateTime::parse_from_rfc2822(
            js_datetime
                .value
                .ok_or(TabError::General)?
                .as_str()
                .ok_or(TabError::General)?,
        )
        .map(|datetime| datetime.with_timezone(&FixedOffset::east(10 * 60 * 60)))
        .or(Err(TabError::General))?)
    }
}

//
// Tab Errors.
//

#[derive(Debug)]
pub enum TabError {
    General,
}

impl std::fmt::Display for TabError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TabError::General => write!(formatter, "Tab operation failed."),
        }
    }
}

impl std::error::Error for TabError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            TabError::General => None,
        }
    }
}

//
// End Tab Errors.
//
