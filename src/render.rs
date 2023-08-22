use wasm_bindgen::prelude::*;
use web_sys::{ Event, HtmlInputElement, EventTarget, InputEvent, Window, Document, HtmlElement };
use crate::log;

pub fn render_excel() -> Result<(), JsValue> {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  let parent = document.create_element("div")?;
  parent.set_attribute("id", "excel_parent_container");
  parent.set_attribute("style", "width:80%;height:80%;overflow:auto;display:flex;flex-direction:row;flex-wrap:wrap;");
  for row in 0..50 {
    for col in 0..50 {
      let row_col_child = document.create_element("div")?;
      row_col_child.set_attribute("id", &format!("excel_{}_{}", row, col));
      row_col_child.set_attribute("style", "border: 1px solid #d1d1d1;font-size:12px;padding:1em;line-height:30px;width:60px;");
      parent.append_child(&row_col_child);
    }
  }
  body.append_child(&parent)?;
  Ok(())
}

pub fn update_excel(row: usize, col: usize, content: &str) {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  
  let excel_td= document.get_element_by_id(&format!("excel_{}_{}", row, col)).unwrap();

  excel_td.set_inner_html(content);
  log(&format!("位于第:{}行,第:{}列，信息为:{}。", row, col, content));
}
