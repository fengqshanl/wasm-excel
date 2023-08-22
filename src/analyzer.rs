use js_sys::{Array, JsString, Uint8Array};
use web_sys::{File, FileReader, HtmlElement};
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use crate::log;
use std::io::Cursor;
use crate::render::update_excel;
use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder, DataType};

pub fn analyzer(file: File) {
  let reader = Rc::new(FileReader::new().unwrap());
  let on_load = Closure::<dyn FnMut()>::new({
    let reader = Rc::clone(&reader); 
    move || {
      let excel_result = reader.result().unwrap();
      let arr = Uint8Array::new(&excel_result);
      let u8arr = convert_to_u8_slice(arr).expect("msg");
      read_data_from_uint8array(&u8arr).unwrap();
  }});
  reader.add_event_listener_with_callback("load", on_load.as_ref().unchecked_ref());
  reader.read_as_array_buffer(&file);
  on_load.forget();
}

fn convert_to_u8_slice(uint8_array: Uint8Array) -> Result<Vec<u8>, JsValue> {
    let array = js_sys::Uint8Array::new(&mut uint8_array.buffer()).subarray(0, uint8_array.length());
    let u8_slice = array.to_vec();
    Ok(u8_slice)
}

fn read_data_from_uint8array(data: &[u8]) -> Result<(), calamine::Error> {
  let cursor = Cursor::new(data);
  let mut xls: Xlsx<_> = calamine::open_workbook_from_rs(cursor)?;
  if let Some(Ok(sheet)) = xls.worksheet_range_at(0) {
    let size = sheet.get_size();
    for row in 0..size.0 {
      for col in 0..size.1 {
        match sheet.get_value((row as u32, col as u32)).unwrap() {
          calamine::DataType::Empty => log("Empty"),
          calamine::DataType::String(x) => {
            update_excel(row, col, x);
          },
          calamine::DataType::Float(x) => log("---------------float--------------->"),
          // 处理其他数据类型
          _ => ()
        }
      }
    }
  }
  Ok(())
}