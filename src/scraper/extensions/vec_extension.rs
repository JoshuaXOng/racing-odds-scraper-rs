use std::fmt;

use crate::models::event::Event;

#[derive(Debug)]
pub struct VecExtension<'a>(pub &'a Vec<Event>);

impl<'a> fmt::Display for VecExtension<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut event_vec_repr = String::from("[");
        let events = self
            .0
            .iter()
            .map(|event| {
                format!(
                    "{{ venue_name={}, planned_start_time={} }}",
                    event.venue_name, event.planned_start_time
                )
            })
            .collect::<Vec<String>>();
        event_vec_repr.push_str(&events.join(", "));
        event_vec_repr.push_str("]");

        write!(formatter, "{}", event_vec_repr.as_str())
    }
}
