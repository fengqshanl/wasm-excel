use wasm_bindgen::prelude::*;
use web_sys::{ Event, HtmlInputElement, EventTarget, InputEvent, Window, Document, HtmlElement, Element, MouseEvent, HtmlDivElement };
use crate::log;

// 构建左右的行列标识边框 滚动方式与container里面的内容一致
pub fn render_border(parent: &Element, secondary_parent: &Element) {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");

  let top_border = render_top_border();

  let left_border = render_left_border();

  parent.append_child(&top_border);
  secondary_parent.append_child(&left_border);
}

pub fn render_top_border() -> Element {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window"); 
  
  let top_border = document.create_element("div").unwrap();

  top_border.set_attribute("id", "excel_top_border");
  top_border.set_attribute("style", "display:flex;flex-direction:row;flex-wrap:no-wrap;");
  for col in 0..51 {
    let row_col_child = document.create_element("div").unwrap();
    row_col_child.set_attribute("id", &format!("excel_top_border_{}", col));
    row_col_child.set_attribute("style", "border: 0.1px solid #d1d1d1;display:flex;font-size:12px;padding:1px;height:15px;min-width:60px;overflow:hidden;text-overflow: ellipsis;white-space: nowrap;justify-content:center;align-items:center");
    row_col_child.set_inner_html(&format!("{}", col));
    top_border.append_child(&row_col_child);
  }
  top_border
}

pub fn render_left_border() -> Element {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window"); 
  
  let left_border = document.create_element("div").unwrap();

  left_border.set_attribute("id", "excel_left_border");
  left_border.set_attribute("style", "display:flex;flex-direction:column;flex-wrap:no-wrap;min-width:60px;margin-top:-1px");
  for col in 1..51 {
    let row_col_child = document.create_element("div").unwrap();
    row_col_child.set_attribute("id", &format!("excel_left_border_{}", col));
    row_col_child.set_attribute("style", "border: 0.1px solid #d1d1d1;display:flex;font-size:12px;padding:1px;height:15px;width:60px;overflow:hidden;text-overflow: ellipsis;white-space: nowrap;justify-content:center;align-items:center");
    row_col_child.set_inner_html(&format!("{}", col));
    left_border.append_child(&row_col_child);
  }
  left_border
}

pub fn render_context(parent: &Element) {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  for row in 0..50 {
    let excel_row = document.create_element("div").unwrap();
    excel_row.set_attribute("id", &format!("excel_{}", row));
    excel_row.set_attribute("class", "excel_row");
    excel_row.set_attribute("style", "display:flex;flex-direction:row;flex-wrap:no-wrap;");
    for col in 0..50 {
      let row_col_child = document.create_element("div").unwrap();
      row_col_child.set_attribute("id", &format!("excel_{}_{}", row, col));
      row_col_child.set_attribute("class", "excel_row_col_div");
      row_col_child.set_attribute("style", "border: 0.1px solid #d1d1d1;display:flex;font-size:12px;padding:1px;height:15px;min-width:60px;overflow:hidden;text-overflow: ellipsis;white-space: nowrap");
      excel_row.append_child(&row_col_child);
    }
    parent.append_child(&excel_row);
  }
}
