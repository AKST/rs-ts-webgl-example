use super::render::api::{
  AttributeKey,
  RenderAPI,
  RenderApiError,
  UniformKey,
};
use super::render::data::{Float32View, DataViewError, View};
use super::render::constants::{
  BufferKind,
  DrawKind,
  ClearMask,
  DrawArrayKind,
  HasBufferKind,
};

#[derive(Clone, Copy, Debug)]
enum VertexAttributes {
  Position,
}

#[derive(Clone, Copy, Debug)]
enum VertexUniforms {
  Resoultion,
}

impl AttributeKey for VertexAttributes {
  fn name(&self) -> &str {
    match self {
      VertexAttributes::Position => "position"
    }
  }
}

impl UniformKey for VertexUniforms {
  fn name(&self) -> &str {
    match self {
      VertexUniforms::Resoultion => "resolution"
    }
  }
}

#[derive(Debug)]
pub struct RenderLoop<R, B> {
  view: Float32View,
  buffer: B,
  context: R,
}

fn triangle_points(width: i32, height: i32) -> [f32; 9] {
  let width_f = width as f32;
  let height_f = height as f32;
  let min_dim = width_f.min(height_f);

  let hr_p = (width_f - min_dim) / 2.0;
  let vt_p = (height_f - min_dim) / 2.0;

  let make_point = |x_offset: f32, y_offset: f32| -> [f32; 3] {
    [
      hr_p + (min_dim * x_offset),
      vt_p + (min_dim * y_offset),
      0.0,
    ]
  };

  let mut result: [f32; 9] = [0.0; 9];
  let left_p = make_point(0.15, 0.15);
  let right_p = make_point(0.85, 0.15);
  let top_p = make_point(0.5, 0.85);

  result[0..3].copy_from_slice(left_p.as_ref());
  result[3..6].copy_from_slice(right_p.as_ref());
  result[6..9].copy_from_slice(top_p.as_ref());

  result
}

impl<R, B> RenderLoop<R, B> where R: RenderAPI<Buffer=B>, B: HasBufferKind {
  pub fn create(context: R, width: i32, height: i32) -> Result<Self, RenderLoopError> {
    let buffer = context.create_buffer(BufferKind::ArrayBuffer)?;
    let data: [f32; 9] = triangle_points(width, height);
    let view = Float32View::create(&data)?;
    context.bind_buffer(&buffer, &view, DrawKind::StaticDraw);

    let position = VertexAttributes::Position;
    let precision = view.get_precision();
    context.vertex_attrib_pointer_with_i32(position, 3, precision, false, 0, 0)?;
    context.enable_vertex_attrib_array(position)?;

    let resolution = VertexUniforms::Resoultion;
    context.uniform2f(resolution, width as f32, height as f32)?;

    Ok(RenderLoop { buffer, view, context })
  }

  pub fn draw(&self) {
    self.context.clear_color(0.0, 0.0, 0.0, 1.0);
    self.context.clear(ClearMask::ColorBufferBit);

    let count = (self.view.length() / 3) as i32;
    self.context.draw_arrays(DrawArrayKind::Triangles, 0, count);
  }

  pub fn update_viewport(&mut self, width: i32, height: i32) -> Result<(), RenderLoopError> {
    let data = triangle_points(width, height);

    self.context.set_viewport(0, 0, width, height);
    self.view.update_data(&data)?;
    self.context.bind_buffer(&self.buffer, &self.view, DrawKind::StaticDraw);

    let resolution = VertexUniforms::Resoultion;
    self.context.uniform2f(resolution, width as f32, height as f32)?;

    return Ok(());
  }
}

pub enum RenderLoopError {
  RenderApiError(RenderApiError),
  DataViewError(DataViewError),
}

impl From<RenderApiError> for RenderLoopError {
  fn from(error: RenderApiError) -> Self {
    RenderLoopError::RenderApiError(error)
  }
}

impl From<DataViewError> for RenderLoopError {
  fn from(error: DataViewError) -> Self {
    RenderLoopError::DataViewError(error)
  }
}

impl ToString for RenderLoopError {
  fn to_string(&self) -> String {
    match self {
      RenderLoopError::RenderApiError(e) => format!("render_loop RenderApiError: {}", e.to_string()),
      RenderLoopError::DataViewError(e) => format!("render_loop DataViewError: {}", e.to_string()),
    }
  }
}
