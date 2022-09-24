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
    hosts::split_hhmm,
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

        let mut store = LoopScheduleStore {
            day_delta: None,
            current_country: None,
            current_venue: None,
        };
        self.loop_schedule_items::<LoopScheduleStore>(
            &mut update_day_delta,
            &mut update_current_country,
            &mut update_venue_name,
            &mut |browser_datetime, s_info_set, store| {
                let venue_event_text = match s_info_set
                    .venue_event
                    .and_then(|venue_event| venue_event.get_inner_text().ok())
                {
                    Some(venue_event_text) => venue_event_text,
                    _ => return Ok(false),
                };
                let event_hhmm = match split_hhmm(venue_event_text.as_str()) {
                    Ok(hhmm) => hhmm,
                    _ => return Ok(false),
                };

                let day_delta = match store.day_delta {
                    Some(day_delta) => day_delta,
                    _ => return Ok(false),
                };
                let event_datetime = match browser_datetime
                    .with_hour(event_hhmm.0)
                    .and_then(|event_datetime| event_datetime.with_minute(event_hhmm.1))
                    .map(|event_datetime| event_datetime + Duration::days(day_delta))
                {
                    Some(event_datetime) => event_datetime,
                    _ => return Ok(false),
                };

                let current_venue = match &store.current_venue {
                    Some(current_venue) => current_venue,
                    _ => return Ok(false),
                };

                event_details.push(Event {
                    venue_name: current_venue.to_owned(),
                    planned_start_time: event_datetime,
                });

                Ok(false)
            },
            &mut store,
        )
        .map_err(|error| {
            ScheduleTabError::General(
                Some(Box::new(error)),
                String::from("Loop of schedule items failed."),
            )
        })?;

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

        let mut store = LoopScheduleStore {
            day_delta: None,
            current_country: None,
            current_venue: None,
        };
        self.loop_schedule_items::<LoopScheduleStore>(
            &mut update_day_delta,
            &mut update_current_country,
            &mut update_venue_name,
            &mut |browser_datetime, s_info_set, store| {
                let venue_event_text = s_info_set
                    .venue_event
                    .and_then(|venue_event| venue_event.get_inner_text().ok())
                    .ok_or(BetfairScheduleTabError::General(
                        None,
                        String::from("Could not get venue event text."),
                    ))?;

                let split_v_name_text = venue_event_text.split(":").collect::<Vec<_>>();
                let (&event_hh, &event_mm) = (
                    split_v_name_text
                        .get(0)
                        .ok_or(BetfairScheduleTabError::General(
                            None,
                            String::from("Could not index hh of split hh:mm."),
                        ))?,
                    split_v_name_text
                        .get(1)
                        .ok_or(BetfairScheduleTabError::General(
                            None,
                            String::from("Could not index mm of split hh:mm."),
                        ))?,
                );

                let day_offset = store.day_delta.ok_or(BetfairScheduleTabError::General(
                    None,
                    String::from("There is no day delta to go off of."),
                ))?;
                let event_datetime = change_datetimes_hhmm(
                    browser_datetime,
                    StrExtension(event_hh),
                    StrExtension(event_mm),
                )
                .map(|event_datetime| event_datetime + Duration::days(day_offset as i64))
                .or(Err(BetfairScheduleTabError::General(
                    None,
                    String::from("Could not construct event's datetime."),
                )))?;

                let event_attributes = s_info_set
                    .venue_event
                    .and_then(|venue_event| venue_event.get_attributes().ok())
                    .ok_or(BetfairScheduleTabError::General(
                        None,
                        String::from("Could not get event element attributes."),
                    ))?
                    .ok_or(BetfairScheduleTabError::General(
                        None,
                        String::from("Could not get event element attributes."),
                    ))?;
                let href_key_index = event_attributes
                    .iter()
                    .position(|attribute| attribute == "href")
                    .ok_or(BetfairScheduleTabError::General(
                        None,
                        String::from("Could not index href key of element."),
                    ))?;
                let href_value_index = href_key_index + 1;
                let event_href = event_attributes.get(href_value_index).ok_or(
                    BetfairScheduleTabError::General(
                        None,
                        String::from("Could not index href value of element."),
                    ),
                )?;

                let venue_name_text = s_info_set
                    .venue_name
                    .and_then(|venue_name| venue_name.get_inner_text().ok())
                    .ok_or(BetfairScheduleTabError::General(
                        None,
                        String::from("Could not get venue name text."),
                    ))?;

                let event_link = EventLink {
                    venue_name: venue_name_text.clone(),
                    event_datetime,
                    navigation_link: String::from(BETFAIR_CONSTANTS.base_url) + &event_href,
                };
                event_links
                    .entry(venue_name_text.clone())
                    .or_insert(vec![event_link.clone()])
                    .push(event_link.clone());

                Ok(false)
            },
            &mut store,
        )?;

        Ok(event_links)
    }

    fn loop_schedule_items<T>(
        &self,
        on_day_change: &mut dyn FnMut(DateTime<FixedOffset>, BetfairScheduleInfoSet, &mut T),
        on_tab_change: &mut dyn FnMut(DateTime<FixedOffset>, BetfairScheduleInfoSet, &mut T),
        on_venue_change: &mut dyn FnMut(DateTime<FixedOffset>, BetfairScheduleInfoSet, &mut T),
        on_event_change: &mut dyn FnMut(
            DateTime<FixedOffset>,
            BetfairScheduleInfoSet,
            &mut T,
        ) -> Result<bool, BetfairScheduleTabError>,
        store: &mut T,
    ) -> Result<(), BetfairScheduleTabError> {
        self.goto_url(BETFAIR_CONSTANTS.racing_url)
            .map_err(|error| {
                BetfairScheduleTabError::General(
                    Some(Box::new(error)),
                    String::from("Could not navigate to betfair schedule page."),
                )
            })?;

        let browser_datetime = self.get_datetime().map_err(|error| {
            BetfairScheduleTabError::General(
                Some(Box::new(error)),
                String::from("Could not compute the datetime of the browser."),
            )
        })?;

        let schedules_days = self
            .get_tab()
            .tab_engine
            .wait_for_elements(format!(".{}", BETFAIR_CSS_CONSTANTS.schedule_day_class).as_str())
            .or(Err(BetfairScheduleTabError::General(
                None,
                String::from("Could not compute the datetime of the browser."),
            )))?;

        let days_to_iterate = schedules_days
            .get(0..=1)
            .ok_or(BetfairScheduleTabError::General(
                None,
                String::from("Could not get the today and tomorrow date tabs."),
            ))?;
        let days_t_i_text = days_to_iterate
            .iter()
            .map(|day| day.get_inner_text().or(Err(())))
            .collect::<Result<Vec<_>, ()>>()
            .or(Err(BetfairScheduleTabError::General(
                None,
                String::from("Could not get the text of date tab."),
            )))?;
        if days_t_i_text[0] != "Today" || days_t_i_text[1] != "Tomorrow" {
            Err(BetfairScheduleTabError::General(
                None,
                String::from("Date tabs are not correct."),
            ))?
        }
        for day_iterating in days_to_iterate {
            day_iterating
                .click()
                .or(Err(BetfairScheduleTabError::General(
                    None,
                    String::from("Could not click the desired date tab."),
                )))?;

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
                .or(Err(BetfairScheduleTabError::General(
                    None,
                    String::from("Could not read country tabs."),
                )))?;
            'outer_most: for country_tab in country_tabs {
                country_tab
                    .click()
                    .or(Err(BetfairScheduleTabError::General(
                        None,
                        String::from("Could not click on country tab."),
                    )))?;

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
                    .or(Err(BetfairScheduleTabError::General(
                        None,
                        String::from("Could not read venue schedules."),
                    )))?;
                for venue_schedule in &venue_schedules {
                    on_venue_change(
                        browser_datetime,
                        BetfairScheduleInfoSet {
                            day_tab: Some(day_iterating),
                            country_tab: Some(country_tab),
                            venue_name: Some(venue_schedule),
                            venue_event: None,
                        },
                        store,
                    );

                    let v_schedule_n = match venue_schedule.get_description() {
                        Ok(v_schedule_n) => v_schedule_n,
                        _ => continue,
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

                    let venue_name = match venue_name.map(|venue_name| {
                        create_element_from_bnid(
                            self.get_tab().tab_engine.as_ref(),
                            venue_name.backend_node_id,
                        )
                    }) {
                        Some(Ok(venue_name)) => venue_name,
                        _ => continue,
                    };

                    for venue_event in venue_events {
                        let venue_event = match create_element_from_bnid(
                            self.get_tab().tab_engine.as_ref(),
                            venue_event.backend_node_id,
                        ) {
                            Ok(venue_event) => venue_event,
                            _ => continue,
                        };

                        if let Ok(true) = on_event_change(
                            browser_datetime,
                            BetfairScheduleInfoSet {
                                day_tab: Some(day_iterating),
                                country_tab: Some(country_tab),
                                venue_name: Some(&venue_name),
                                venue_event: Some(&venue_event),
                            },
                            store,
                        ) {
                            break 'outer_most;
                        };
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
        self.navigation_link.contains(BETFAIR_CONSTANTS.base_url)
    }
}

pub struct BetfairScheduleInfoSet<'a> {
    pub day_tab: Option<&'a Element<'a>>,
    pub country_tab: Option<&'a Element<'a>>,
    pub venue_name: Option<&'a Element<'a>>,
    pub venue_event: Option<&'a Element<'a>>,
}

struct LoopScheduleStore {
    day_delta: Option<i64>,
    current_country: Option<String>,
    current_venue: Option<String>,
}

fn update_day_delta(
    _: DateTime<FixedOffset>,
    s_info_set: BetfairScheduleInfoSet,
    store: &mut LoopScheduleStore,
) {
    let s_days_delta = match s_info_set
        .day_tab
        .map(|selected_day| selected_day.get_inner_text())
    {
        Some(Ok(s_day_text)) if s_day_text == String::from("Today") => 0,
        Some(Ok(s_day_text)) if s_day_text == String::from("Tomorrow") => 1,
        _ => return,
    };
    store.day_delta = Some(s_days_delta);
}

fn update_current_country(
    _: DateTime<FixedOffset>,
    s_info_set: BetfairScheduleInfoSet,
    store: &mut LoopScheduleStore,
) {
    if let Some(country_name) = s_info_set
        .country_tab
        .and_then(|country_tab| country_tab.get_inner_text().ok())
    {
        store.current_country = Some(country_name)
    }
}

fn update_venue_name(
    _: DateTime<FixedOffset>,
    s_info_set: BetfairScheduleInfoSet,
    store: &mut LoopScheduleStore,
) {
    if let Some(v_name_text) = s_info_set
        .venue_name
        .and_then(|venue_name| venue_name.get_inner_text().ok())
    {
        store.current_venue = Some(v_name_text);
    }
}

#[allow(dead_code)]
struct ScheduleCache {}
#[allow(dead_code)]
impl ScheduleCache {
    fn is_invalid(&self) {}
    fn perform_refresh(&self) {}
}

//
// End Miscellaneous Items.
//

//
// BetfairScheduleTab Errors
//

#[derive(Debug)]
pub enum BetfairScheduleTabError {
    General(Option<Box<dyn std::error::Error + 'static>>, String),
}

impl std::fmt::Display for BetfairScheduleTabError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BetfairScheduleTabError::General(_, message) => write!(formatter, "{}", message),
        }
    }
}

impl std::error::Error for BetfairScheduleTabError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BetfairScheduleTabError::General(source, _) => source.as_deref(),
        }
    }
}

//
// End BetfairScheduleTab Errors
//
