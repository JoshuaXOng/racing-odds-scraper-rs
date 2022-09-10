use std::{path::Path};

use chromiumoxide::{page::{Page as PageEngine}, cdp::browser_protocol::page::{CaptureScreenshotParams, CaptureScreenshotFormat}, error::CdpError};

use async_trait::async_trait;

pub struct Page {
  pub engine: PageEngine,
  pub target_url: Option<String>
}

#[async_trait]
pub trait AsPage {
  fn get_page(&self) -> &Page;

  async fn take_screenshot(&self, save_to_path: &Path) -> Result<(), CdpError> {
    self.get_page().engine.save_screenshot(
      CaptureScreenshotParams::builder()
        .format(CaptureScreenshotFormat::Png)
        .build(), 
      save_to_path
    ).await
      .and_then(|_| Ok(()))
  }

  async fn refresh_page(&self) -> Result<(), CdpError> {
    self.get_page().engine.reload().await
      .and_then(|_| Ok(()))
  }

  async fn get_datetime(&self) -> Result<(), CdpError> {
    self.get_page().engine.evaluate_function("() => new Date()").await
      .and_then(|today| 
        today.into_value()
          .or_else(|error| Err(CdpError::Serde(error)))
      )
  }

  async fn check_if_drift(&self) -> Result<bool, CdpError> {
    match (self.get_page().engine.url().await, self.get_page().target_url) {
      (Ok(Some(current_url)), Some(target_url)) => Ok(current_url == target_url),
      (Err(error), _) => Err(error),
      _ => Ok(false)
    }
  }

  async fn fake_mouse_movement() {
    
  }
}
