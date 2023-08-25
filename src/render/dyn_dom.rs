use wasm_bindgen::prelude::*;
use web_sys::{ Event, HtmlInputElement, EventTarget, InputEvent, Window, Document, HtmlElement, Element, MouseEvent, HtmlDivElement };
use crate::log;
use super::util::calculate_position;

pub fn dyn_hover(current_id: &str) {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");

  let hover_div = document.get_element_by_id(current_id).unwrap();

  match document.get_element_by_id("dyn_user_hover_div") {
    Some(tar) => {
      log("document - create");
      let (top, left) = calculate_position(&tar, &hover_div);
      tar.set_attribute("style", &format!("position:absolute;border:4px solid blue;height:15px;min-width:60px;top:{}px;left:{}px", top, left));
    },
    _ => {
      log("document - 2");
      let user_hover_div = document.create_element("div").unwrap();
      user_hover_div.set_attribute("id", "dyn_user_hover_div");
      let (top, left) = calculate_position(&user_hover_div, &hover_div);
      hover_div.set_attribute("style", &format!("position:absolute;border:4px solid blue;height:15px;min-width:60px;top:{}px;left:{}px", top, left));
      body.append_child(&user_hover_div);
    }
  } 
}