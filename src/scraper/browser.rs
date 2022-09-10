use chromiumoxide::{Browser as BrowserEngine, BrowserConfig, Handler, cdp::browser_protocol::target::CreateTargetParams};

pub struct Browser {
  pub engine: BrowserEngine,
  handler: Handler
}

impl Browser {
  pub async fn new(browser_config: Option<BrowserConfig>) -> Result<Self, BrowserError> {
    let browser_config = match browser_config {
      None => BrowserConfig::builder().with_head().build().or_else(|error| Err(BrowserError::OpenBrowser))?,
      Some(browser_config) => browser_config
    };

    let (browser_engine, handler) = BrowserEngine::launch(browser_config).await
      .map_err(|_| BrowserError::OpenBrowser)?;

    Ok(Browser {
      engine: browser_engine,
      handler: handler
    })
  }

  async fn close_pages(&self, urls_to_close: Vec<&str>, exempted_urls: Vec<&str>) -> Result<(), BrowserError> {
    for existing_page in self.engine.pages().await.map_err(|_| BrowserError::ClosePage)? {
      let page_url = existing_page.url().await.map_err(|_| BrowserError::ClosePage)?
        .ok_or(BrowserError::ClosePage)?;
      
      if !exempted_urls.contains(&page_url.as_str()) && urls_to_close.contains(&page_url.as_str()) {
        existing_page.close().await.map_err(|_| BrowserError::ClosePage)?
      } 
    }

    Ok(())
  }

  async fn open_pages(&self, urls_to_open: Vec<&str>) -> Result<(), BrowserError> {
    for url_to_open in urls_to_open {
      self.engine.new_page(CreateTargetParams::new(url_to_open)).await
        .map_err(|_| BrowserError::OpenPage)?;
    }

    Ok(())
  }

  async fn clone(&self) -> Result<Self, BrowserError> {
    let new_browser = match self.engine.config() {
      Some(from_config) => Browser::new(Some(from_config.clone())).await,
      None => Err(BrowserError::OpenBrowser)?
    };
    
    let from_pages = self.engine.pages().await
      .map_err(|_| BrowserError::OpenBrowser)?;
      
    for from_page in &from_pages {
      let url = from_page.url().await
        .map_err(|_| BrowserError::OpenBrowser)
        .and_then(|url| match url {
          Some(url) => Ok(url),
          None => Err(BrowserError::OpenBrowser)
        })?;
      
      new_browser.as_ref().unwrap().engine.new_page(
        CreateTargetParams::new(url)
      ).await
        .map_err(|_| BrowserError::OpenBrowser)?;
    };

    new_browser
  }
}

#[derive(Debug)]
enum BrowserError {
  OpenBrowser,
  OpenPage,
  ClosePage,
}


impl std::fmt::Display for BrowserError {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      BrowserError::OpenBrowser => write!(formatter, "Failed to create new browser instance."),
      BrowserError::OpenPage => write!(formatter, "Failed to open a new page instance."),
      BrowserError::ClosePage => write!(formatter, "Failed to close a new page instance."),
    }
  }
}

impl std::error::Error for BrowserError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      BrowserError::OpenBrowser => None,
      BrowserError::OpenPage => None,
      BrowserError::ClosePage => None,
    }
  }
}