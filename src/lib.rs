#![allow(unused_variables)]
pub mod analyzer;
pub mod render;
pub mod event;
pub mod style;
use wasm_bindgen::prelude::*;
use web_sys::{ Event, HtmlInputElement, EventTarget, InputEvent };
use js_sys::Function;
use js_sys::Object;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    style::load_style();
    let val = document.create_element("input")?;
    val.set_attribute("type", "file");
    let set_page = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
      let target = event.target().unwrap();
      let tar = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target).expect("msg");
      let file_list: web_sys::FileList = HtmlInputElement::files(tar).expect("msg");
      for i in 0..file_list.length() {
        if let Some(file) = file_list.item(i) {
          analyzer::analyzer(file);
        }
      }
    });
    val.add_event_listener_with_callback("change", set_page.as_ref().unchecked_ref()).unwrap();
    set_page.forget();
    body.append_child(&val)?;
    render::render_excel()?; 
    Ok(())
}