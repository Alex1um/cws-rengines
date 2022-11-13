use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use rustc_hash::FxHashMap;
use crate::events::event::{Event};
use crate::events::event_provider::EventProvider;
use crate::renders::base::render::Render;
use std::thread::sleep;
use std::time::Duration;
use crate::renders::sdl::scene::SceneRef;

#[cfg(target_os = "emscripten")]
extern "C" {
  fn emscripten_sleep(ms: u32);
}

impl EventProvider for Vec<Event> {
  fn provide_events(&mut self, buf: &mut Vec<Event>) {
    buf.extend(self.drain(..));
  }
}

pub type InLoopProviderRef<'a> = &'a mut Vec<Event>;

type EventCallBack = Box<dyn FnMut(&Event, InLoopProviderRef)>;

pub struct EventLoop<'a, T: Render + Sized> {
  scene: SceneRef<'a>,
  render: T,
  event_listeners: FxHashMap<Event, Vec<EventCallBack>>,
  event_providers: Vec<Rc<RefCell<dyn EventProvider>>>,
}

impl<T> EventLoop<'_, T> where T: Render + Sized {
  pub fn new(scene: SceneRef, render: T) -> EventLoop<T> {
    EventLoop {
      scene,
      render,
      event_listeners: FxHashMap::<Event, Vec<EventCallBack>>::default(),
      event_providers: vec![],
    }
  }

  pub fn get_render(&mut self) -> &mut T {
    return &mut self.render;
  }

  pub fn add_event_listener(&mut self, event: Event, f: EventCallBack) -> Result<(), Box<dyn Error>> {
    if let Some(v) = self.event_listeners.get_mut(&event) {
      v.push(f);
    } else {
      self.event_listeners.insert(event, vec![f]);
    }
    Ok(())
  }

  pub fn add_event_provider(&mut self, provider: Rc<RefCell<dyn EventProvider>>) {
    self.event_providers.push(provider);
  }

  pub fn start(&mut self) {
    let mut buf: Vec<Event> = vec![];
    let mut inlp = Vec::<Event>::new();
    'main_loop: loop {
      for provider in &self.event_providers {
        provider.borrow_mut().provide_events(&mut buf);
      }
      buf.extend(inlp.drain(..));
      for e in buf.drain(..) {
        if let Some(listeners) = self.event_listeners.get_mut(&e) {
          for listener in listeners {
            listener(&e, &mut inlp);
          }
        }
        if e == Event::Exit {
          break 'main_loop;
        }
      }
      if let Some(listeners) = self.event_listeners.get_mut(&Event::Loop) {
        for listener in listeners {
          listener(&Event::Loop, &mut inlp);
        }
      }
      self.render.render(&self.scene);

      #[cfg(not(target_os = "emscripten"))]
      sleep(Duration::from_millis(100));

      #[cfg(target_os = "emscripten")]
      unsafe { emscripten_sleep(100); }
    }
  }
}