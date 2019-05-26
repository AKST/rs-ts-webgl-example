use super::context::{AttributeKey, RenderAPI, RenderApiError};
use super::data::{Float32View, DataViewError, View};
use super::constants::{
  BufferKind,
  DrawKind,
  ClearMask,
  DrawArrayKind,
  HasBufferKind,
};

#[derive(Clone, Copy, Debug)]
enum Attributes {
  Position,
}

impl AttributeKey for Attributes {
  fn name(&self) -> &str {
    match self {
      Attributes::Position => "position"
    }
  }
}

#[derive(Debug)]
pub struct Render<R, B> {
  view: Float32View,
  buffer: B,
  context: R,
}

impl<R, B> Render<R, B> where R: RenderAPI<Buffer=B>, B: HasBufferKind {
  pub fn create(context: R) -> Result<Self, RenderInitialisationError> {
    let buffer = context.create_buffer(BufferKind::ArrayBuffer)?;
    let data: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    let view = Float32View::create(&data)?;
    context.bind_buffer(&buffer, &view, DrawKind::StaticDraw);

    let position = Attributes::Position;
    let precision = view.get_precision();
    context.vertex_attrib_pointer_with_i32(position, 3, precision, false, 0, 0)?;
    context.enable_vertex_attrib_array(position)?;

    Ok(Render { buffer, view, context })
  }

  pub fn draw(&self) {
    self.context.clear_color(0.0, 0.0, 0.0, 1.0);
    self.context.clear(ClearMask::ColorBufferBit);

    let count = (self.view.length() / 3) as i32;
    self.context.draw_arrays(DrawArrayKind::Triangles, 0, count);
  }
}

pub enum RenderInitialisationError {
  RenderApiError(RenderApiError),
  DataViewError(DataViewError),
}

impl From<RenderApiError> for RenderInitialisationError {
  fn from(error: RenderApiError) -> RenderInitialisationError {
    RenderInitialisationError::RenderApiError(error)
  }
}

impl From<DataViewError> for RenderInitialisationError {
  fn from(error: DataViewError) -> RenderInitialisationError {
    RenderInitialisationError::DataViewError(error)
  }
}

impl RenderInitialisationError {
  pub fn to_string(self) -> String {
    match self {
      RenderInitialisationError::RenderApiError(e) => format!("RenderApiError: {}", e.to_string()),
      RenderInitialisationError::DataViewError(e) => format!("DataViewError: {}", e.to_string()),
    }
  }
}
