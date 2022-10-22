use std::error::Error;
use rustc_hash::FxHashMap;
use crate::events::event::{Event};
use crate::events::event_provider::EventProvider;
use crate::objects::area::AreaRef;
use crate::renders::base::render::Render;
use std::thread::sleep;
use std::time::Duration;
use crate::renders::sdl::render::{Scene, Window};

#[cfg(target_arch = "wasm32")]
extern "C" {
  fn emscripten_sleep(ms: u32);
}

type EventCallBack = Box<dyn FnMut()>;

pub struct EventLoop<'a, T: Render + Sized + EventProvider> {
  scene: Scene<'a>,
  window: &'a mut Window,
  render: T,
  event_listeners: FxHashMap<Event, Vec<EventCallBack>>,
}

impl<T> EventLoop<'_, T> where T: Render + Sized + EventProvider {
  pub fn new<'a>(scene: Scene<'a>, render: T, window: &'a mut Window) -> EventLoop<'a, T> {
    EventLoop {
      scene,
      window,
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
      self.window.provide_events(&mut buf);
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
      self.render.render(&self.scene);

      #[cfg(not(target_arch = "wasm32"))]
      sleep(Duration::from_millis(200));

      #[cfg(target_arch = "wasm32")]
      unsafe { emscripten_sleep(200); }
    }
  }
}