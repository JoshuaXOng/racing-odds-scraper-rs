mod hosts; 
mod pages;
mod readers;
mod browser_group;
mod browser;

use crate::browser::{Browser};
use async_std::stream::StreamExt;
use crate::hosts::betfair::betfair_page::BetfairPage;
use crate::pages::page::{Page, AsPage};
use crate::hosts::betfair::betfair_schedule_page::{BetfairSchedulePage};
use crate::pages::schedule_page::SchedulePage;

// #[async_std::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//   // let mut b = Browser::new(None).await?;
//   // let handle = async_std::task::spawn(async move {
//   //   loop {
//   //     let _event = b.handler.next().await.unwrap();
//   //   }
//   // });

//   // let p = 
//   // b.engine.new_page("https://www.youtube.com/").await?;
//   // let s = BetfairSchedulePage::new(p, String::from("https://en.wikipedia.org"));
  
//   // handle.await;
  
//   let mut c = Browser::new(None).await?;
//   let handle2 = async_std::task::spawn(async move {
//     loop {
//       let _event = c.handler.next().await.unwrap();
//     }
//   });

//   // let p = 
//   c.engine.new_page("https://en.wikipedia.org").await?;

//   (handle2.await);
//   // (handle2.await, handle.await);
//   Ok(())
// }

use headless_chrome::{Browser as BB, LaunchOptions};
use headless_chrome::protocol::cdp::Page as PP;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let browser = BB::new(
    LaunchOptions::default_builder()
      .headless(false)  
      .build()
      .expect("Could not find chrome-executable"),
  )?;

  let tab = browser.wait_for_initial_tab()?;

  tab.navigate_to("https://www.wikipedia.org")?;

  tab.wait_for_element("input#searchInput")?.click()?;

  tab.type_str("WebKit")?.press_key("Enter")?;

  let browsera = BB::new(
    LaunchOptions::default_builder()
      .headless(false)  
      .build()
      .expect("Could not find chrome-executable"),
  )?;

  let taba = browsera.wait_for_initial_tab()?;

  taba.navigate_to("https://www.wikipedia.org")?;

  taba.wait_for_element("input#searchInput")?.click()?;

  taba.type_str("WebKit")?.press_key("Enter")?;

  Ok(())
}