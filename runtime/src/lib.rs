mod render;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast};
use web_sys::{WebGlRenderingContext};
use render::builder::{RenderBuilder, WebRender};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Runtime {
  render: WebRender,
}

#[wasm_bindgen]
impl Runtime {
  fn new(render: WebRender) -> Self {
    Runtime { render }
  }

  #[wasm_bindgen]
  pub fn tick(&self) {
    self.render.draw();
  }

  #[wasm_bindgen(js_name = "debugState")]
  pub fn debug_state(&self) {
    console_log!("Debug: {:#?}", self);
  }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct RuntimeBuilder {
  render_builder: RenderBuilder,
}

#[wasm_bindgen]
impl RuntimeBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Result<RuntimeBuilder, JsValue> {
    let render_builder = RenderBuilder::new();
    Ok(RuntimeBuilder { render_builder })
  }

  #[wasm_bindgen(js_name = "linkWebglContext")]
  pub fn link_webgl_context(&mut self, maybe_context: JsValue) -> Result<(), JsValue> {
    return maybe_context.dyn_into::<WebGlRenderingContext>()
      .map(|context| self.render_builder.set_context(context))
      .map_err(|value| {
        let message = format!("expected web gl context, instead got {:?}", value);
        return JsValue::from_str(message.as_ref())
      });
  }

  #[wasm_bindgen(js_name = "linkFragShader")]
  pub fn link_frag_shader(&mut self, shader_source: &str) -> Result<(), JsValue> {
    return self.render_builder.set_frag_shader(shader_source)
      .map_err(|err| JsValue::from_str(err.to_string().as_ref()))
  }

  #[wasm_bindgen(js_name = "linkVertShader")]
  pub fn link_vert_shader(&mut self, shader_source: &str) -> Result<(), JsValue> {
    return self.render_builder.set_vert_shader(shader_source)
      .map_err(|err| JsValue::from_str(err.to_string().as_ref()))
  }

  #[wasm_bindgen(js_name = "createRuntime")]
  pub fn create_runtime(&mut self) -> Result<Runtime, JsValue> {
    let render = self.render_builder.build_render()
      .map_err(|err| JsValue::from_str(err.to_string().as_ref()))?;
    Ok(Runtime::new(render))
  }

  #[wasm_bindgen(js_name = "debugState")]
  pub fn debug_state(&self) {
    console_log!("Debug: {:#?}", self);
  }
}


#[wasm_bindgen(js_name = "setupPanicHook")]
pub fn setup_panic_hook() {
    console_error_panic_hook::set_once();
}
