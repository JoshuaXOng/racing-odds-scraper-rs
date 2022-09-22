#[derive(Clone)]
pub struct BetfairConstants {
    pub base_url: &'static str,
    pub racing_url: &'static str,
}

pub static BETFAIR_CONSTANTS: BetfairConstants = BetfairConstants {
    base_url: "https://www.betfair.com.au/exchange/plus/",
    racing_url: "https://www.betfair.com.au/exchange/plus/en/horse-racing-betting-7/",
};

pub struct BetfairCSSConstants {
    pub schedule_class: &'static str,
    pub schedule_day_class: &'static str,
    pub schedule_tab_class: &'static str,
    pub venue_schedule_class: &'static str,
    pub venue_name_class: &'static str,
    pub venue_event_class: &'static str,
    pub contestant_entry_class: &'static str,
}

pub static BETFAIR_CSS_CONSTANTS: BetfairCSSConstants = BetfairCSSConstants {
    schedule_class: "mod-todays-racing",
    schedule_day_class: "schedule-filter-button",
    schedule_tab_class: "tab-wrapper",
    venue_schedule_class: "meeting-item",
    venue_name_class: "meeting-label",
    venue_event_class: "race-link",
    contestant_entry_class: "runner-line",
};
