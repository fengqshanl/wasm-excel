use web_sys::{ Element, DragEvent, Event };
use wasm_bindgen::prelude::*;
use crate::log;

pub fn resize_divide(index: &i32) -> Element {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");

  let row_col_child_divide = document.create_element("div").unwrap();
  row_col_child_divide.set_attribute("class", &format!("excel_left_border_divide excel_left_border_divide_{}", index));
  {
    let index = index.clone();
    let drag = Closure::<dyn FnMut(DragEvent)>::new(move |event: DragEvent| {
      let offset = -event.offset_y();
      let before = document.get_element_by_id(&format!("excel_left_border_{}", index.clone())).unwrap();
      before.set_attribute("style", &format!("height: {}px", offset + 15)).unwrap();
    });
    row_col_child_divide.add_event_listener_with_callback("drag", drag.as_ref().unchecked_ref()).unwrap();
    drag.forget();
  }

  row_col_child_divide 
}