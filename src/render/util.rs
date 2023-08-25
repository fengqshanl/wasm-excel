use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlDivElement};
use crate::log;

pub fn calculate_position(current_element: &Element, tar_element: &Element) -> (i32, i32) {
  let tar = tar_element.clone().dyn_into::<HtmlDivElement>().expect("msg");
  let top = tar.offset_top();
  let left = tar.offset_left();
  (top, left)
}