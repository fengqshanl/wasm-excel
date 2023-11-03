use wasm_bindgen::prelude::*;
use crate::event::basic;
use super::resize;
use web_sys::{ Event, HtmlInputElement, EventTarget, InputEvent, Window, Document, HtmlElement, Element, MouseEvent, HtmlDivElement };
use crate::log;

// 构建左右的行列标识边框 滚动方式与container里面的内容一致
pub fn render_border(parent: &Element) {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");

  let top_border = render_top_border();

  let left_border = render_left_border();

  parent.append_child(&top_border);
  parent.append_child(&left_border);
}

pub fn render_top_border() -> Element {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");

  let top_border = document.create_element("div").unwrap();

  top_border.set_attribute("id", "excel_top_border");
  for col in 0..51 {
    let row_col_child = document.create_element("div").unwrap();
    row_col_child.set_attribute("id", &format!("excel_top_border_{}", col));
    row_col_child.set_attribute("class", "excel_top_border_col");
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
  for col in 1..5001 {
    let row_col_child = document.create_element("div").unwrap();
    row_col_child.set_attribute("id", &format!("excel_left_border_{}", col));
    row_col_child.set_attribute("class", "excel_left_border_col");
    row_col_child.set_inner_html(&format!("{}", col));

    // let divide = resize::resize_divide(&col);

    left_border.append_child(&row_col_child);
    // left_border.append_child(&divide);
  }
  left_border
}

pub fn render_context(parent: &Element) {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let context_container = document.create_element("table").unwrap();
  context_container.set_attribute("class", "table is-bordered is-striped is-narrow is-hoverable is-fullwidth");
  for row in 0..5000 {
    let excel_row = document.create_element("tr").unwrap();
    for col in 0..50 {
      let row_col_child = document.create_element("td").unwrap();
      row_col_child.set_attribute("id", &format!("excel_{}_{}", row, col));
      excel_row.append_child(&row_col_child);
    }
    context_container.append_child(&excel_row);
  }
  parent.append_child(&context_container);
}
