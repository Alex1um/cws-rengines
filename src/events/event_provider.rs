use crate::events::event::Event;

pub trait EventProvider {

  fn provide_events(&mut self, buf: &mut Vec<Event>);


}