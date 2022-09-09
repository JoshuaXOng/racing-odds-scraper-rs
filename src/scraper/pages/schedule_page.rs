use super::page::{Page, AsPage};

use async_trait::async_trait;
use chromiumoxide::error::CdpError;

pub struct SchedulePage {
  pub page: Page,
  pub target_page: String,
}

#[async_trait]
pub trait AsSchedulePage: AsPage {
  fn get_schedule_page(&self) -> &SchedulePage;

  async fn setup_page(&self) -> Result<bool, CdpError> {
    self.get_schedule_page().page.engine.goto(
      self.get_schedule_page().target_page
    ).await?
      .wait_for_navigation().await?;

    Ok(true)
  }

  async fn perform_scrape(&self);
}
