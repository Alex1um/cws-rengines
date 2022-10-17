use std::error::Error;
use rustc_hash::FxHashMap;
use crate::events::event::{Event};
use crate::events::event_provider::EventProvider;
use crate::objects::area::AreaRef;
use crate::renders::base::render::Render;
use std::thread::sleep;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
extern "C" {
  fn emscripten_sleep(ms: u32);
}

type EventCallBack = Box<dyn FnMut()>;

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
    if let Some(v) = self.event_listeners.get_mut(&event) {
      v.push(f);
    } else {
      self.event_listeners.insert(event, vec![f]);
    }
    Ok(())
  }

  pub fn start(&mut self) {
    let mut buf: Vec<Event> = vec![];
    'main_loop: loop {
      self.render.provide_events(&mut buf);
      for e in buf.drain(0..buf.len()) {
        if let Some(listeners) = self.event_listeners.get_mut(&e) {
          for listener in listeners {
            // listener(&mut self.render as &mut dyn Render);
            listener();
          }
        }
      }
      if let Some(listeners) = self.event_listeners.get_mut(&Event::Loop) {
        for listener in listeners {
          // listener(&mut self.render as &mut dyn Render);
          listener();
        }
      }
      self.render.render();

      #[cfg(not(target_arch = "wasm32"))]
      sleep(Duration::from_millis(200));

      #[cfg(target_arch = "wasm32")]
      unsafe { emscripten_sleep(200); }
    }
  }
}