pub mod update;
pub mod basic;
pub mod util;
pub mod resize;

use wasm_bindgen::prelude::*;
use web_sys::{ Event, HtmlInputElement, EventTarget, InputEvent, Window, Document, HtmlElement, Element, MouseEvent, HtmlDivElement };
use crate::log;

pub fn render_excel() -> Result<(), JsValue> {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  let parent = document.create_element("div")?;
  parent.set_attribute("id", "excel_parent_container");
  basic::render_border(&parent);
  basic::render_context(&parent);
  body.append_child(&parent)?;
  Ok(())
}

