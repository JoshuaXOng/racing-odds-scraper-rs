use chromiumoxide::{Browser as BrowserEngine, BrowserConfig, Handler, cdp::browser_protocol::target::CreateTargetParams};

struct Browser {
  engine: BrowserEngine,
  handler: Handler
}

impl Browser {
  async fn new(browser_config: Option<BrowserConfig>) -> Result<Self, BrowserError> {
    let browser_config = match browser_config {
      None => BrowserConfig::builder().with_head().build().or_else(|error| Err(BrowserError::Init))?,
      Some(browser_config) => browser_config
    };

    let (browser_engine, handler) = BrowserEngine::launch(browser_config).await
      .map_err(|_| BrowserError::Init)?;

    Ok(Browser {
      engine: browser_engine,
      handler: handler
    })
  }

  fn setup(&self) {
    
  }

  async fn close_most_pages(&self) {
    
  }

  async fn open_pages(&self) {

  }

  async fn clone(&self) -> Result<Self, BrowserError> {
    let new_browser = match self.engine.config() {
      Some(from_config) => Browser::new(Some(from_config.clone())).await,
      None => Err(BrowserError::Init)?
    };
    
    let from_pages = self.engine.pages().await
      .map_err(|_| BrowserError::Init)?;
      
    for from_page in &from_pages {
      let url = from_page.url().await
        .map_err(|_| BrowserError::Init)
        .and_then(|url| match url {
          Some(url) => Ok(url),
          None => Err(BrowserError::Init)
        })?;
      
      new_browser.as_ref().unwrap().engine.new_page(
        CreateTargetParams::new(url)
      ).await
        .map_err(|_| BrowserError::Init)?;
    };

    new_browser
  }
}

#[derive(Debug)]
enum BrowserError {
  Init,
}

impl std::fmt::Display for BrowserError {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      BrowserError::Init => write!(formatter, "Failed to create new browser instance."),
    }
  }
}

impl std::error::Error for BrowserError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      BrowserError::Init => None,
    }
  }
}