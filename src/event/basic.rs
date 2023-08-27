use web_sys::{ Element, HtmlDivElement, Event };
use wasm_bindgen::prelude::*;
use crate::log;

pub fn init_basic_event(tar: &Element) {
  let set_contenteditable = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        log("set_contenteditable");
        let target = event.target().unwrap();
        let tar = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlDivElement>(&target).expect("msg");
        tar.set_attribute("contenteditable", "true");
      });
  tar.add_event_listener_with_callback("click", set_contenteditable.as_ref().unchecked_ref()).unwrap();
  set_contenteditable.forget();
}