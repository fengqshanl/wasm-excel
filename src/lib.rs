
#![allow(unused_variables)]
fn main() {
use wasm_bindgen::prelude::*;
use web_sys::InputEvent;
use js_sys::Function;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("input")?;
    val.set_attribute("type", "file");
    let set_page = Closure::<dyn FnMut(InputEvent)>::new(move |target: InputEvent| {
      let data = target.data().expect("msg");
      println!("data: {:?}", data) 
    });
    val.add_event_listener_with_callback("change", set_page.as_ref().unchecked_ref()).unwrap();
    set_page.forget();
    body.append_child(&val)?;

    Ok(())
}

}