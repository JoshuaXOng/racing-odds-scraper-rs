use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Duration, FixedOffset, Timelike};
use headless_chrome::{protocol::cdp::DOM::Node, Element, Tab as TabEngine};

use super::{
    betfair_constants::{BETFAIR_CONSTANTS, BETFAIR_CSS_CONSTANTS},
    betfair_tab::{AsBetfairTab, AsTab, BetfairTab, Tab},
};
use crate::{
    extensions::{
        datetime_extensions::change_datetimes_hhmm,
        rhc_extensions::{create_element_from_bnid, for_each_node},
        str_extension::StrExtension,
    },
    tabs::schedule_tab::{AsScheduleTab, Event, ScheduleTab, ScheduleTabError},
};

pub struct BetfairScheduleTab {
    pub betfair_tab: BetfairTab,
    pub schedule_tab: ScheduleTab,
}

impl AsBetfairTab for BetfairScheduleTab {
    fn get_betfair_tab(&self) -> &BetfairTab {
        &self.betfair_tab
    }
}

impl AsTab for BetfairScheduleTab {
    fn get_tab(&self) -> &Tab {
        &self.betfair_tab.get_tab()
    }
}

impl AsScheduleTab for BetfairScheduleTab {
    fn get_schedule_tab(&self) -> &ScheduleTab {
        &self.schedule_tab
    }

    fn scrape_schedule(&self) -> Result<Vec<Event>, ScheduleTabError> {
        let mut event_details = vec![] as Vec<Event>;

        // self.loop_schedule_items(&mut |captured_datetime, info_set| {
        //     let (venue_name_text, day_selected_text, venue_event_text) =
        //         if let (Ok(venue_name_text), Ok(day_selected_text), Ok(venue_event_text)) = (
        //             info_set.venue_name.get_inner_text(),
        //             info_set.day_selected.get_inner_text(),
        //             info_set.venue_event.get_inner_text(),
        //         ) {
        //             (venue_name_text, day_selected_text, venue_event_text)
        //         } else {
        //             Err(BetfairScheduleTabError::General)?
        //         };

        //     let event_time = venue_event_text.split(":").collect::<Vec<_>>();

        //     let (event_time_hour, event_time_min) =
        //         if let (Some(Ok(event_time_hour)), Some(Ok(event_time_min))) = (
        //             event_time
        //                 .get(0)
        //                 .map(|event_time_hour| event_time_hour.parse::<u32>()),
        //             event_time
        //                 .get(1)
        //                 .map(|event_time_min| event_time_min.parse::<u32>()),
        //         ) {
        //             (event_time_hour, event_time_min)
        //         } else {
        //             Err(BetfairScheduleTabError::General)?
        //         };

        //     let event_start_datetime = if let Some(event_start_datetime) = captured_datetime
        //         .with_hour(event_time_hour)
        //         .and_then(|event_start_datetime| event_start_datetime.with_minute(event_time_min))
        //     {
        //         let days_schedule_ahead = match day_selected_text.as_str() {
        //             "Today" => 0,
        //             "Tomorrow" => 1,
        //             _ => Err(BetfairScheduleTabError::General)?,
        //         };
        //         event_start_datetime + Duration::days(days_schedule_ahead as i64)
        //     } else {
        //         Err(BetfairScheduleTabError::General)?
        //     };

        //     event_details.push(Event {
        //         venue_name: venue_name_text,
        //         planned_start_time: event_start_datetime,
        //     });

        //     Ok(())
        // })
        // .or(Err(ScheduleTabError::BadScrape))?;

        Ok(event_details)
    }
}

impl BetfairScheduleTab {
    pub fn new(tab_engine: Arc<TabEngine>) -> Self {
        Self {
            betfair_tab: BetfairTab::new(tab_engine.clone()),
            schedule_tab: ScheduleTab::new(tab_engine.clone()),
        }
    }

    pub fn get_event_links(
        &self,
    ) -> Result<HashMap<String, Vec<EventLink>>, BetfairScheduleTabError> {
        let mut event_links = HashMap::from([]);

		let mut store = HashMap::from([]) as HashMap<&str, &str>;
		self.loop_schedule_items::<HashMap<&str, &str>>(
			&mut |a, b, c| {
				*c.entry("key").or_insert("value") = "value";
			}, 
			&mut |a, b, c| {}, 
			&mut |a, b, c| {}, 
			&mut |a, b, c| {}, 
			&mut store
		)?;
        // self.loop_schedule_items(&mut |datetime_of_scrape, s_info_set| {
        //     let venue_name_text = s_info_set
        //         .venue_event
        //         .get_inner_text()
        //         .or(Err(BetfairScheduleTabError::General))?;

        //     let venue_event_text = s_info_set
        //         .venue_event
        //         .get_inner_text()
        //         .or(Err(BetfairScheduleTabError::General))?;

        //     s_info_set
        //         .venue_event
        //         .get_inner_text()
        //         .or(Err(BetfairScheduleTabError::General))?;

        //     let split_v_name_text = venue_event_text.split(":").collect::<Vec<_>>();
        //     let (&event_hh, &event_mm) = (
        //         split_v_name_text
        //             .get(0)
        //             .ok_or(BetfairScheduleTabError::General)?,
        //         split_v_name_text
        //             .get(1)
        //             .ok_or(BetfairScheduleTabError::General)?,
        //     );

        //     let s_days_delta = match s_info_set.day_selected.get_inner_text() {
        //         Ok(s_day_text) if s_day_text == String::from("Today") => 0,
        //         Ok(s_day_text) if s_day_text == String::from("Tomorrow") => 1,
        //         _ => Err(BetfairScheduleTabError::General)?,
        //     };

        //     let event_datetime = change_datetimes_hhmm(
        //         datetime_of_scrape,
        //         StrExtension(event_hh),
        //         StrExtension(event_mm),
        //     )
        //     .map(|event_datetime| event_datetime + Duration::days(s_days_delta as i64))
        //     .or(Err(BetfairScheduleTabError::General))?;

        //     let event_attributes = s_info_set
        //         .venue_event
        //         .get_attributes()
        //         .or(Err(BetfairScheduleTabError::General))?
        //         .ok_or(BetfairScheduleTabError::General)?;
        //     let href_key_index = event_attributes
        //         .iter()
        //         .position(|attribute| attribute == "href")
        //         .ok_or(BetfairScheduleTabError::General)?;
        //     let href_value_index = href_key_index + 1;
        //     let event_href = event_attributes
        //         .get(href_value_index)
        //         .ok_or(BetfairScheduleTabError::General)?;

        //     let event_link = EventLink {
        //         venue_name: venue_name_text.clone(),
        //         event_datetime,
        //         navigation_link: String::from(BETFAIR_CONSTANTS.base_url) + &event_href,
        //     };
        //     event_links
        //         .entry(venue_name_text.clone())
        //         .or_insert(vec![event_link.clone()])
        //         .push(event_link.clone());
        // })?;

        Ok(event_links)
    }

    fn loop_schedule_items<T>(
        &self,
        on_day_change: &mut dyn FnMut(DateTime<FixedOffset>, BetfairScheduleInfoSet, &mut T),
        on_tab_change: &mut dyn FnMut(DateTime<FixedOffset>, BetfairScheduleInfoSet, &mut T),
        on_venue_change: &mut dyn FnMut(DateTime<FixedOffset>, BetfairScheduleInfoSet, &mut T),
        on_event_change: &mut dyn FnMut(DateTime<FixedOffset>, BetfairScheduleInfoSet, &mut T),
        store: &mut T,
    ) -> Result<(), BetfairScheduleTabError> {
        self.goto_url(BETFAIR_CONSTANTS.racing_url)
            .or(Err(BetfairScheduleTabError::General))?;

        let browser_datetime = self
            .get_datetime()
            .or(Err(BetfairScheduleTabError::General))?;

        let schedules_days = self
            .get_tab()
            .tab_engine
            .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.schedule_day_class).as_str())
            .or(Err(BetfairScheduleTabError::General))?;

        schedules_days
            .get(0)
            .map(|first_day| {
                first_day
                    .get_inner_text()
                    .map(|f_day_text| f_day_text == "Today")
                    .unwrap_or(false)
                    && first_day.click().is_ok()
            })
            .ok_or(BetfairScheduleTabError::General)?;

        let days_to_iterate = schedules_days
            .get(0..=1)
            .ok_or(BetfairScheduleTabError::General)?;
        let days_t_i_text = days_to_iterate
            .iter()
            .map(|day| day.get_inner_text().or(Err(())))
            .collect::<Result<Vec<_>, ()>>()
            .or(Err(BetfairScheduleTabError::General))?;
        if days_t_i_text[0] != "Today" || days_t_i_text[1] != "Tomorrow" {
            Err(BetfairScheduleTabError::General)?
        }
        for day_iterating in days_to_iterate {
            day_iterating
                .click()
                .or(Err(BetfairScheduleTabError::General))?;

            on_day_change(
                browser_datetime,
                BetfairScheduleInfoSet {
                    day_tab: Some(day_iterating),
                    country_tab: None,
                    venue_name: None,
                    venue_event: None,
                },
                store,
            );

            let country_tabs = &self
                .get_tab()
                .tab_engine
                .wait_for_elements(
                    format!(".{}", BETFAIR_CSS_CONSTANTS.schedule_tab_class).as_str(),
                )
                .or(Err(BetfairScheduleTabError::General))?;
            for country_tab in country_tabs {
                country_tab
                    .click()
                    .or(Err(BetfairScheduleTabError::General))?;

                on_tab_change(
                    browser_datetime,
                    BetfairScheduleInfoSet {
                        day_tab: Some(day_iterating),
                        country_tab: Some(country_tab),
                        venue_name: None,
                        venue_event: None,
                    },
                    store,
                );

                let venue_schedules = self
                    .get_tab()
                    .tab_engine
                    .wait_for_elements(
                        format!(".{}", BETFAIR_CSS_CONSTANTS.venue_schedule_class).as_str(),
                    )
                    .or(Err(BetfairScheduleTabError::General))?;
                for venue_schedule in &venue_schedules {
                    on_tab_change(
                        browser_datetime,
                        BetfairScheduleInfoSet {
                            day_tab: Some(day_iterating),
                            country_tab: Some(country_tab),
                            venue_name: Some(venue_schedule),
                            venue_event: None,
                        },
                        store,
                    );

                    let v_schedule_n = if let Ok(v_schedule_n) = venue_schedule.get_description() {
                        v_schedule_n
                    } else {
                        continue;
                    };

                    let mut venue_name = None as Option<Node>;
                    let mut venue_events = vec![] as Vec<Node>;
                    for_each_node(&v_schedule_n, &mut |node: &Node| {
                        if node.local_name == "div"
                            && node
                                .attributes
                                .as_ref()
                                .map(|attributes| {
                                    attributes.contains(&String::from(
                                        BETFAIR_CSS_CONSTANTS.venue_name_class,
                                    ))
                                })
                                .unwrap_or(false)
                        {
                            venue_name = Some(node.clone());
                        }

                        if node.local_name == "a"
                            && node
                                .attributes
                                .as_ref()
                                .map(|attributes| {
                                    attributes.contains(&String::from(
                                        BETFAIR_CSS_CONSTANTS.venue_event_class,
                                    ))
                                })
                                .unwrap_or(false)
                        {
                            venue_events.push(node.clone());
                        }
                    });

                    let venue_name = if let Some(venue_name) = venue_name {
                        create_element_from_bnid(
                            self.get_tab().tab_engine.as_ref(),
                            venue_name.backend_node_id,
                        )
                    } else {
                        continue;
                    };

                    for venue_event in venue_events {
                        let venue_event = if let Ok(venue_event) = create_element_from_bnid(
                            self.get_tab().tab_engine.as_ref(),
                            venue_event.backend_node_id,
                        ) {
                            venue_event
                        } else {
                            continue;
                        };

                        on_event_change(
                            browser_datetime,
                            BetfairScheduleInfoSet {
                                day_tab: Some(day_iterating),
                                country_tab: Some(country_tab),
                                venue_name: Some(venue_schedule),
                                venue_event: Some(&venue_event),
                            },
                            store,
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

//
// Miscellaneous Items.
//

#[derive(Debug, Clone)]
pub struct EventLink {
    pub venue_name: String,
    pub event_datetime: DateTime<FixedOffset>,
    pub navigation_link: String,
}

#[allow(dead_code)]
impl EventLink {
    pub fn validate_uri(&self) -> bool {
        self.navigation_link.contains("betfair")
    }
}

pub struct BetfairScheduleInfoSet<'a> {
    pub day_tab: Option<&'a Element<'a>>,
    pub country_tab: Option<&'a Element<'a>>,
    pub venue_name: Option<&'a Element<'a>>,
    pub venue_event: Option<&'a Element<'a>>,
}

//
// End Miscellaneous Items.
//

//
// BetfairScheduleTab Errors
//

#[derive(Debug)]
pub enum BetfairScheduleTabError {
    General,
}

//
// End BetfairScheduleTab Errors
//
