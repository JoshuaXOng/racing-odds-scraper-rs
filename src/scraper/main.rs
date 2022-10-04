use std::error::Error;

use browser::{Browser, Host, TabType};
use chrono::{DateTime, FixedOffset, NaiveDate};
use env_logger::fmt::Color;
use log::Level;
use std::io::Write;
use tracing_subscriber::FmtSubscriber;

mod browser;
mod extensions;
mod hosts;
mod models;
mod orchestrators;
mod tabs;

fn main() -> Result<(), Box<dyn Error>> {
    setup_tracer();

    let mut main_browser = Browser::new().unwrap();

    main_browser
        .open_page((TabType::Schedule, Host::Betfair))
        .unwrap();
    if let Some(schedule_tab) = main_browser.schedule_tabs.get(&Host::Betfair) {
        println!("{:?}", schedule_tab.scrape_schedule().unwrap());
    }

    Ok(())
}

#[allow(dead_code)]
fn stage_1() {
    let mut main_browser = Browser::new().unwrap();

    main_browser
        .open_page((TabType::Events, Host::Betfair))
        .unwrap();
    if let Some(events_tab) = main_browser.events_tabs.get(&Host::Betfair) {
        println!(
            "{:?}",
            events_tab
                .scrape_event(
                    "Sandown",
                    DateTime::from_local(
                        NaiveDate::from_ymd(2022, 9, 25).and_hms(17, 30, 0),
                        FixedOffset::east(10 * 60 * 60)
                    )
                )
                .unwrap()
        );
    };
}

#[allow(dead_code)]
#[tracing::instrument]
fn setup_logger() {
    tracing::info!("Setting up logger.");
    env_logger::Builder::from_default_env()
        .format(|formatter, record| {
            let level = record.level();
            let mut style = formatter.style();
            match record.level() {
                Level::Error => style.set_color(Color::Red),
                Level::Warn => style.set_color(Color::Yellow),
                Level::Info => style.set_color(Color::Green),
                Level::Debug => style.set_color(Color::Blue),
                Level::Trace => style.set_color(Color::Cyan),
            };
            
            writeln!(
                formatter,
                "{}:{} {} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.target(),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                style.value(level),
                record.args()
            )
        })
        .init();
    tracing::info!("Finished Setting up logger.");
}

fn setup_tracer() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}
