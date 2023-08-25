use wasm_bindgen::prelude::*;
use web_sys::{ Event, HtmlInputElement, EventTarget, InputEvent, Window, Document, HtmlElement, Element, MouseEvent, HtmlDivElement };
use crate::log;

pub fn update_excel(row: usize, col: usize, content: &str) {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");
  
  let excel_td= document.get_element_by_id(&format!("excel_{}_{}", row, col)).unwrap();

  excel_td.set_inner_html(content);
}