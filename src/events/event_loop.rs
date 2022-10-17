use std::error::Error;
use rustc_hash::FxHashMap;
use crate::events::event::{Event};
use crate::events::event_provider::EventProvider;
use crate::objects::area::AreaRef;
use crate::renders::base::render::Render;

type EventCallBack = fn();

pub struct EventLoop<T: Render + Sized + EventProvider> {
  area: AreaRef,
  render: T,
  event_listeners: FxHashMap<Event, Vec<EventCallBack>>,
}

impl<T> EventLoop<T> where T: Render + Sized + EventProvider {
  pub fn new(area: AreaRef, render: T) -> EventLoop<T> {
    EventLoop {
      area,
      render,
      event_listeners: FxHashMap::<Event, Vec<EventCallBack>>::default(),
    }
  }

  pub fn add_event_listener(&mut self, event: Event, f: EventCallBack) -> Result<(), Box<dyn Error>> {
    let v = self.event_listeners.get_mut(&event).expect("Correct event");
    v.push(f);
    Ok(())
  }

  pub fn start(&mut self) {
    let mut buf: Vec<Event> = vec![];
    loop {
      self.render.provide_events(&mut buf);
      for e in buf.drain(0..buf.len()) {
        if let Some(listeners) = self.event_listeners.get(&e) {
          for listener in listeners {
            listener();
          }
        }
      }
    }
  }
}