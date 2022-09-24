use std::sync::Arc;

use chrono::{DateTime, Duration, FixedOffset};
use headless_chrome::Tab as TabEngine;

use super::{
    betfair_constants::BETFAIR_CSS_CONSTANTS,
    betfair_schedule_tab::BetfairScheduleTab,
    betfair_tab::{AsBetfairTab, AsTab, BetfairTab, Tab},
};
use crate::{
    extensions::rhc_extensions::{create_element_from_bnid, for_each_node, is_node_of_class},
    models::odds::Odds,
    tabs::events_tab::{AsEventsTab, ContestantOdds, EventsTab, EventsTabError},
};

pub struct BetfairEventsTab {
    pub betfair_tab: BetfairTab,
    pub events_tab: EventsTab,
    schedule_tab: BetfairScheduleTab,
}

impl AsBetfairTab for BetfairEventsTab {
    fn get_betfair_tab(&self) -> &BetfairTab {
        &self.betfair_tab
    }
}

impl AsTab for BetfairEventsTab {
    fn get_tab(&self) -> &Tab {
        self.betfair_tab.get_tab()
    }
}

impl AsEventsTab for BetfairEventsTab {
    fn get_events_tab(&self) -> &EventsTab {
        &self.events_tab
    }

    fn scrape_event(
        &self,
        venue_name: &str,
        event_time: DateTime<FixedOffset>,
    ) -> Result<Vec<ContestantOdds>, EventsTabError> {
        let event_links = self.schedule_tab.get_event_links().map_err(|error| {
            EventsTabError::General(
                Some(Box::new(error)),
                String::from("Could not get event links from schedule."),
            )
        })?;

        let v_event_links = event_links.get(venue_name).ok_or_else(|| EventsTabError::General(
            None,
            String::from("Could not key venue specific event links from schedule."),
        ))?;

        let v_event_link = v_event_links
            .iter()
            .find(|v_event_link| {
                v_event_link.event_datetime - Duration::minutes(2) <= event_time
                    && v_event_link.event_datetime + Duration::minutes(2) >= event_time
            })
            .ok_or_else(|| EventsTabError::General(None, String::from("Could not get link to event.")))?;

        self.goto_url(v_event_link.navigation_link.as_str())
            .map_err(|error| {
                EventsTabError::General(
                    Some(Box::new(error)),
                    String::from("Could not navigate to the event's url."),
                )
            })?;

        let mut contestant_odds = vec![];

        let contestant_entries = &self
            .get_tab()
            .tab_engine
            .wait_for_elements(
                format!(".{}", BETFAIR_CSS_CONSTANTS.contestant_entry_class).as_str(),
            )
            .map_err(|_| EventsTabError::General(
                None,
                String::from("Could not scrape contestant entries from betting table."),
            ))?;
        for contestant_entry in contestant_entries {
            let c_entry_node = match contestant_entry.get_description() {
                Ok(c_entry_node) => c_entry_node.to_owned(),
                _ => continue,
            };

            let mut contestant_name = None as Option<String>;
            let mut back_entries = vec![] as Vec<Odds>;
            let mut lay_entries = vec![] as Vec<Odds>;
            for_each_node(&c_entry_node, &mut |node| {
                if is_node_of_class(node, "runner-name") {
                    contestant_name =
                        create_element_from_bnid(&self.get_tab().tab_engine, node.backend_node_id)
                            .and_then(|a| a.get_inner_text().or(Err(())))
                            .ok();
                }

                let is_back_button = is_node_of_class(node, "back-button");
                let is_lay_button = is_node_of_class(node, "lay-button");
                if is_back_button || is_lay_button {
                    let entry_text = match create_element_from_bnid(
                        &self.get_tab().tab_engine,
                        node.backend_node_id,
                    )
                    .and_then(|entry| entry.get_inner_text().or(Err(())))
                    {
                        Ok(entry_text) => entry_text,
                        _ => return,
                    };

                    let odds_and_money = entry_text.split_whitespace().collect::<Vec<_>>();
                    let back_odds = odds_and_money.first();
                    let back_money = odds_and_money.get(1);

                    if let (Some(&odds), Some(&money)) = (back_odds, back_money) {
                        let entry_container = match is_back_button {
                            true => &mut back_entries,
                            false => &mut lay_entries,
                        };
                        if let Ok(odds) = (odds, money).try_into() as Result<Odds, ()> {
                            entry_container.push(odds);
                        }
                    }
                }
            });

            if let Some(contestant_name) = contestant_name {
                contestant_odds.push(ContestantOdds {
                    contestant_name,
                    backing_odds: back_entries,
                    laying_odds: lay_entries,
                })
            }
        }

        Ok(contestant_odds)
    }
}

impl BetfairEventsTab {
    pub fn new(tab_engine: Arc<TabEngine>) -> Self {
        Self {
            betfair_tab: BetfairTab::new(tab_engine.clone()),
            events_tab: EventsTab::new(tab_engine.clone()),
            schedule_tab: BetfairScheduleTab::new(tab_engine),
        }
    }
}
