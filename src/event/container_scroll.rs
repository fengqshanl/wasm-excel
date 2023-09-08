use web_sys::Element;

pub fn container_scroll_init(element: &Element) {
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
  val.add_event_listener_with_callback("scroll", set_page.as_ref().unchecked_ref()).unwrap();
  set_page.forget();
}