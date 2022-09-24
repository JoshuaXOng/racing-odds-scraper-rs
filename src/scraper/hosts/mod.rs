use std::str::FromStr;

pub mod betfair;

fn split_hhmm<T: FromStr>(hhmm: &str) -> Result<(T, T), ()> {
    let split_hhmm = hhmm.split(":").collect::<Vec<_>>();
    Ok((
        split_hhmm
            .get(0)
            .and_then(|hh| hh.parse::<T>().ok())
            .ok_or(())?,
        split_hhmm
            .get(1)
            .and_then(|mm| mm.parse::<T>().ok())
            .ok_or(())?,
    ))
}
