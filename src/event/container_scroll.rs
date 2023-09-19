use web_sys::{ Element, Event, HtmlDivElement };
use wasm_bindgen::prelude::*;
use crate::log;

pub fn container_scroll_init(element: &Element) {
  let scroll_event = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
    let target = event.target().unwrap();
    let tar = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlDivElement>(&target).expect("msg");
    log(&format!("scroll_top: {}, scroll_left: {}", tar.scroll_top(), tar.scroll_left()));
  });
  element.add_event_listener_with_callback("scroll", scroll_event.as_ref().unchecked_ref()).unwrap();
  scroll_event.forget();
}