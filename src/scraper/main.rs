mod hosts; 
mod pages;
mod readers;
mod browser_group;
mod browser;

use crate::browser::Browser;
use crate::hosts::betfair::betfair_page::BetfairPage;
use crate::pages::page::{Page, AsPage};
use crate::hosts::betfair::betfair_schedule_page::{BetfairSchedulePage};
use crate::pages::schedule_page::SchedulePage;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Starting Racing-Odds-Scraper-Rs");
  
  let b = Browser::new(None).await?;
  let p = b.engine.new_page("https://en.wikipedia.org").await?;
  let s = BetfairSchedulePage::new(p, String::from("https://en.wikipedia.org"));
  
  println!("Exited Racing-Odds-Scraper-Rs");

  Ok(())
}
