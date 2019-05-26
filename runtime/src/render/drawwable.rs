use web_sys::WebGlRenderingContext;

pub trait Drawwable {
  fn draw(&self, context: &WebGlRenderingContext) -> Result<(), DrawError>;
}

pub enum DrawError {
}
