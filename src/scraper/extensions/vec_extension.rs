use std::fmt;

use crate::models::event::Event;

pub struct VecExtension<'a>(pub &'a Vec<Event>);

impl<'a> fmt::Display for VecExtension<'a> {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let mut event_vec_repr = String::from("[");
    self.0.iter().for_each(|event| {
      event_vec_repr.push_str(
        format!(
          "{{ venue_name={}, planned_start_time={}, has_started={} }}", 
          event.venue_name, 
          event.planned_start_time,
          event.has_started
        ).as_str()
      )
    });
    event_vec_repr.push_str("]");

    write!(formatter, "{}", event_vec_repr.as_str())
  }
}
