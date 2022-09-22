use std::error::Error;

use browser::{Browser, Host, TabType};
use chrono::{DateTime, FixedOffset, NaiveDate};

mod browser;
mod extensions;
mod hosts;
mod models;
mod orchestrators;
mod tabs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut main_browser = Browser::new().unwrap();

    main_browser
        .open_page((TabType::Events, Host::Betfair))
        .unwrap();
    if let Some(events_tab) = main_browser.events_tabs.get(&Host::Betfair) {
        println!(
            "{:?}",
            events_tab
                .scrape_event(
                    "Turffontein",
                    DateTime::from_local(
                        NaiveDate::from_ymd(2022, 9, 22).and_hms(23, 0, 0),
                        FixedOffset::east(10 * 60 * 60)
                    )
                )
                .unwrap()
        );
    }

    // main_browser.open_page((TabType::Schedule, Host::Betfair)).unwrap();
    // if let Some(schedule_tab) = main_browser.schedule_tabs.get(&Host::Betfair) {
    //   println!("{:?}", schedule_tab.scrape_schedule().unwrap());
    // }

    Ok(())
}
