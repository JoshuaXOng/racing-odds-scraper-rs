use std::sync::{Arc, MutexGuard, TryLockError};

use headless_chrome::{Browser as BrowserEngine, Tab as TabEngine};

pub trait Extended {
    fn get_current_urls(
        &self,
    ) -> Result<Vec<String>, TryLockErrMGuard<'_, TabEngVec>>;
}

impl Extended for BrowserEngine {
    fn get_current_urls(
        &self,
    ) -> Result<Vec<String>, TryLockErrMGuard<'_, TabEngVec>> {
        let current_pages = self.get_tabs().try_lock()?;
        Ok(current_pages
            .iter()
            .map(|current_page| current_page.get_url())
            .collect::<Vec<String>>())
    }
}

//
// Miscellaneous Items.
//

type TryLockErrMGuard<'a, T> = TryLockError<MutexGuard<'a, T>>;

type TabEngVec = Vec<Arc<TabEngine>>;

//
// Miscellaneous Items.
//