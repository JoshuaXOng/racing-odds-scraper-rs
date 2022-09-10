use std::sync::Arc;

use super::page::{Page, AsPage};

use async_trait::async_trait;
use chromiumoxide::error::CdpError;

pub struct SchedulePage<'a> {
  pub page: Page,
  pub target_url: Arc<&'a String>,
}

#[async_trait]
pub trait AsSchedulePage: AsPage {
  fn get_schedule_page(&self) -> &SchedulePage;

  async fn setup_page(&self) -> Result<bool, CdpError> {
    self.get_schedule_page().page.engine.goto(
      self.get_schedule_page().target_url.as_ref().clone()
    ).await?
      .wait_for_navigation().await?;

    Ok(true)
  }

  fn perform_scrape(&self) -> i32;
}
