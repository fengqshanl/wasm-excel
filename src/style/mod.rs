pub fn load_style() {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let body = document.body().expect("document should have a body");

  let link = document.create_element("link").unwrap();
  link.set_attribute("rel", "stylesheet");
  link.set_attribute("href", "index.css");
  body.append_child(&link);
}