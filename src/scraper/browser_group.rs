use crate::browser::Browser;

pub struct BrowserGroup {
  primary_browser: Browser,
  backup_browsers: Vec<Browser>,
}

impl BrowserGroup {
  fn backup_primary(&mut self) -> Result<(), BrowserGroupError> {
    let new_browser = self.primary_browser.clone().map_err(|_| BrowserGroupError::CreateBackup)?;
    self.backup_browsers.push(new_browser);
    Ok(())
  }

  fn dequeue_backup(&mut self) -> Result<(), BrowserGroupError> {
    if self.backup_browsers.len() >= 1 {
      self.backup_browsers.remove(0);
      Ok(())
    } else {
      Err(BrowserGroupError::DeleteBackup)
    }
  }
}

enum BrowserGroupError {
  CreateBackup,
  DeleteBackup
}