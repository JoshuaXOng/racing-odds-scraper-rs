use chrono::{DateTime, FixedOffset, Timelike};

pub fn change_datetimes_hhmm<T: TryInto<u32>>(
    current_datetime: DateTime<FixedOffset>,
    target_hour: T,
    target_min: T,
) -> Result<DateTime<FixedOffset>, ()> {
    let parsed_hhmm = if let (Ok(target_hour), Ok(target_min)) =
        (target_hour.try_into(), target_min.try_into())
    {
        (target_hour, target_min)
    } else {
        Err(())?
    };

    current_datetime
        .with_hour(parsed_hhmm.0)
        .and_then(|target_datetime| target_datetime.with_minute(parsed_hhmm.1))
        .ok_or(())
}
