pub mod update;
pub mod basic;
pub mod util;
pub mod dyn_dom;

use wasm_bindgen::prelude::*;
use web_sys::{ Event, HtmlInputElement, EventTarget, InputEvent, Window, Document, HtmlElement, Element, MouseEvent, HtmlDivElement };
use crate::log;

pub fn render_excel() -> Result<(), JsValue> {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  let parent = document.create_element("div")?;
  parent.set_attribute("id", "excel_parent_container");
  parent.set_attribute("style", "width:90vw;height:80vh;overflow:auto;display:flex;flex-direction:column");
  let secondary_parent = document.create_element("div")?;
  secondary_parent.set_attribute("id", "excel_secondary_parent_container");
  secondary_parent.set_attribute("style", "width:100%;display:flex;flex-direction:row");
  let third_parent = document.create_element("div")?;
  third_parent.set_attribute("id", "excel_third_parent_container");
  third_parent.set_attribute("style", "width:100%;display:flex;flex-direction:column;margin-left:3px;margin-top:-1px");
  basic::render_border(&parent, &secondary_parent);
  basic::render_context(&third_parent);
  let on_click = Closure::<dyn FnMut(MouseEvent)>::new({
  move |e: MouseEvent| {
    match e.target() {
      Some(target) => {
        let tar: &Element = wasm_bindgen::JsCast::dyn_ref::<web_sys::Element>(&target).expect("msg");
        let id = tar.id();
        dyn_dom::dyn_hover(&id);
      },
      _ => log("no target")
    }
  }});
  third_parent.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref());
  on_click.forget();
  secondary_parent.append_child(&third_parent);
  parent.append_child(&secondary_parent);
  body.append_child(&parent)?;
  Ok(())
}

