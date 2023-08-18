use js_sys::{Array, JsString, Uint8Array};
use web_sys::{File, FileReader};
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder, DataType};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn analyzer(file: File) {
  let reader = Rc::new(FileReader::new().unwrap());
  let on_load = Closure::<dyn FnMut()>::new({
    let reader = Rc::clone(&reader); 
    move || {
      let excel_result = reader.result().unwrap();
      let arr = Uint8Array::new(&excel_result);
      let mut excel = open_workbook(arr).map_err(|e| JsValue::from_str(&format!("{:?}", e))).unwrap();
      let mut result = String::new();
      if let Some(Ok(range)) = excel.worksheet_range_at(0) {
          for row in range.rows() {
              for cell in row.iter() {
                  match cell {
                      DataType::String(s) => result.push_str(&format!("{} | ", s)),
                      DataType::Float(f) => result.push_str(&format!("{} | ", f)),
                      _ => result.push_str("Unsupported | "),
                  }
              }
              result.push('\n');
          }
      }
      log(&result);
  }});
  reader.add_event_listener_with_callback("load", on_load.as_ref().unchecked_ref());
  reader.read_as_array_buffer(&file);
  on_load.forget();
}