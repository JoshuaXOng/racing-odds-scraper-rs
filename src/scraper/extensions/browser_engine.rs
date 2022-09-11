use std::sync::{MutexGuard, TryLockError, Arc};

use headless_chrome::{Browser as BrowserEngine, Tab as TabEngine};

pub trait Extended {
  fn get_current_urls(&self) -> Result<Vec<String>, TryLockError<MutexGuard<'_, Vec<Arc<TabEngine>>>>>;
}

impl Extended for BrowserEngine {
  fn get_current_urls(&self) -> Result<Vec<String>, TryLockError<MutexGuard<'_, Vec<Arc<TabEngine>>>>> {
    let current_pages = self.get_tabs().try_lock()?;
    Ok(current_pages.iter().map(|current_page| current_page.get_url()).collect::<Vec<String>>())
  }
}
