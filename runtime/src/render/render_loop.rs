use super::api::{AttributeKey, RenderAPI, RenderApiError};
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
pub struct RenderLoop<R, B> {
  view: Float32View,
  buffer: B,
  context: R,
}

impl<R, B> RenderLoop<R, B> where R: RenderAPI<Buffer=B>, B: HasBufferKind {
  pub fn create(context: R) -> Result<Self, RenderLoopInitError> {
    let buffer = context.create_buffer(BufferKind::ArrayBuffer)?;
    let data: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    let view = Float32View::create(&data)?;
    context.bind_buffer(&buffer, &view, DrawKind::StaticDraw);

    let position = Attributes::Position;
    let precision = view.get_precision();
    context.vertex_attrib_pointer_with_i32(position, 3, precision, false, 0, 0)?;
    context.enable_vertex_attrib_array(position)?;

    Ok(RenderLoop { buffer, view, context })
  }

  pub fn draw(&self) {
    self.context.clear_color(0.0, 0.0, 0.0, 1.0);
    self.context.clear(ClearMask::ColorBufferBit);

    let count = (self.view.length() / 3) as i32;
    self.context.draw_arrays(DrawArrayKind::Triangles, 0, count);
  }
}

pub enum RenderLoopInitError {
  RenderApiError(RenderApiError),
  DataViewError(DataViewError),
}

impl From<RenderApiError> for RenderLoopInitError {
  fn from(error: RenderApiError) -> Self {
    RenderLoopInitError::RenderApiError(error)
  }
}

impl From<DataViewError> for RenderLoopInitError {
  fn from(error: DataViewError) -> Self {
    RenderLoopInitError::DataViewError(error)
  }
}

impl RenderLoopInitError {
  pub fn to_string(self) -> String {
    match self {
      RenderLoopInitError::RenderApiError(e) => format!("RenderApiError: {}", e.to_string()),
      RenderLoopInitError::DataViewError(e) => format!("DataViewError: {}", e.to_string()),
    }
  }
}
