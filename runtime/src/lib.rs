use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");
    let body = document.body().expect("should have a body");

    {
      let text = document.create_text_node("and hello from rust.");
      body.append_child(&text)?;
    }

    Ok(())
}
