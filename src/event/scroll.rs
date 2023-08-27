use web_sys::{ Element, HtmlDivElement, Event };
use wasm_bindgen::prelude::*;
use crate::log;

pub fn scroll_event(tar: &Element) {
  log("scroll_event");
  let scroll_callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        log("set_contenteditable");
        let target = event.target().unwrap();
        let tar = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlDivElement>(&target).expect("msg");
        tar.set_attribute("contenteditable", "true");
      });
  tar.add_event_listener_with_callback("wheel", scroll_callback.as_ref().unchecked_ref()).unwrap();
  scroll_callback.forget();
}