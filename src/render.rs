use wasm_bindgen::prelude::*;
use web_sys::{ Event, HtmlInputElement, EventTarget, InputEvent, Window, Document, HtmlElement, Element };
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
  render_border(&parent, &secondary_parent);
  render_context(&third_parent);
  secondary_parent.append_child(&third_parent);
  parent.append_child(&secondary_parent);
  body.append_child(&parent)?;
  Ok(())
}

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
    excel_row.set_attribute("style", "display:flex;flex-direction:row;flex-wrap:no-wrap;");
    for col in 0..50 {
      let row_col_child = document.create_element("div").unwrap();
      row_col_child.set_attribute("id", &format!("excel_{}_{}", row, col));
      row_col_child.set_attribute("style", "border: 0.1px solid #d1d1d1;display:flex;font-size:12px;padding:1px;height:15px;min-width:60px;overflow:hidden;text-overflow: ellipsis;white-space: nowrap");
      excel_row.append_child(&row_col_child);
    }
    parent.append_child(&excel_row);
  }
}

pub fn update_excel(row: usize, col: usize, content: &str) {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  
  let excel_td= document.get_element_by_id(&format!("excel_{}_{}", row, col)).unwrap();

  excel_td.set_inner_html(content);
  log(&format!("位于第:{}行,第:{}列，信息为:{}。", row, col, content));
}
