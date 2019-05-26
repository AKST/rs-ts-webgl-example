use std::convert::TryFrom;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlBuffer};
use super::constants::{
  BufferKind,
  ClearMask,
  DrawArrayKind,
  DrawKind,
  ViewPrecision,
  HasBufferKind,
  HasViewPrecision,
  HasClearMaskKind,
  HasDrawArrayKind,
  HasDrawKind,
};
use super::data::{View};

type AttributeValue = u32;
type UniformValue = u32;

pub trait AttributeKey {
  fn name(&self) -> &str;
}

pub trait UniformKey {
  fn name(&self) -> &str;
}

pub trait IntoAttributeValue {
  fn with_context<C>(self, context: &C) -> Result<AttributeValue, RenderApiError> where C: RenderAPI;
}

pub trait RenderAPI {
  type Buffer: HasBufferKind;

  fn create_buffer(
      &self,
      kind: BufferKind,
  ) -> Result<Self::Buffer, RenderApiError>;

  fn bind_buffer<V>(
      &self,
      buffer: &Self::Buffer,
      view: &V,
      draw_kind: DrawKind,
  ) where V: View;

  fn get_attribute<AK>(&self, key: AK) -> Result<AttributeValue, RenderApiError>
      where AK: AttributeKey;

  fn vertex_attrib_pointer_with_i32<A>(
      &self,
      key: A,
      size: i32,
      precision: ViewPrecision,
      normalized: bool,
      stride: i32,
      offset: i32
  ) -> Result<(), RenderApiError> where A: IntoAttributeValue;

  fn enable_vertex_attrib_array<A>(&self, key: A) -> Result<(), RenderApiError>
      where A: IntoAttributeValue;

  fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32);

  fn clear(&self, mask: ClearMask);

  fn draw_arrays(&self, mode: DrawArrayKind, first: i32, count: i32);
}

#[derive(Debug)]
pub struct WebRenderAPI {
  gl: WebGlRenderingContext,
  program: WebGlProgram,
}

impl WebRenderAPI {
  pub fn create(gl: WebGlRenderingContext, program: WebGlProgram) -> Self {
    WebRenderAPI { gl, program }
  }
}

impl RenderAPI for WebRenderAPI {
  type Buffer = WebRenderBuffer;

  fn create_buffer(
      &self,
      kind: BufferKind,
  ) -> Result<Self::Buffer, RenderApiError> {
    self.gl.create_buffer().ok_or(RenderApiError::FailedToCreateBuffer).map(|internal| {
      WebRenderBuffer { kind, internal }
    })
  }

  fn bind_buffer<V>(
      &self,
      buffer: &Self::Buffer,
      view: &V,
      draw_kind: DrawKind,
  ) where V: View {
    let kind = buffer.buffer_kind_constant();
    let draw = draw_kind.draw_kind_constant();
    self.gl.bind_buffer(kind, Some(&buffer.internal));
    self.gl.buffer_data_with_array_buffer_view(kind, view.object(), draw)
  }

  fn get_attribute<AK>(&self, key: AK) -> Result<AttributeValue, RenderApiError> where AK: AttributeKey {
    let name = key.name();
    let glint = self.gl.get_attrib_location(&self.program, name);
    u32::try_from(glint).map_err(|_| RenderApiError::InvalidAttributeName(name.to_string()))
  }

  fn vertex_attrib_pointer_with_i32<A>(
      &self,
      key: A,
      size: i32,
      precision: ViewPrecision,
      normalized: bool,
      stride: i32,
      offset: i32
  ) -> Result<(), RenderApiError> where A: IntoAttributeValue {
    key.with_context(self).map(|index| {
      self.gl.vertex_attrib_pointer_with_i32(
          index,
          size,
          precision.view_precision_constant(),
          normalized,
          stride,
          offset,
      )
    })
  }

  fn enable_vertex_attrib_array<A>(&self, key: A) -> Result<(), RenderApiError> where A: IntoAttributeValue {
    key.with_context(self).map(|i| self.gl.enable_vertex_attrib_array(i))
  }

  fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
    self.gl.clear_color(red, green, blue, alpha);
  }

  fn clear(&self, mask: ClearMask) {
    self.gl.clear(mask.clear_mask_constant());
  }

  fn draw_arrays(&self, mode: DrawArrayKind, first: i32, count: i32) {
    self.gl.draw_arrays(mode.draw_array_kind_constant(), first, count);
  }
}

#[derive(Clone, Debug)]
pub struct WebRenderBuffer {
  pub kind: BufferKind,
  pub internal: WebGlBuffer,
}

impl HasBufferKind for WebRenderBuffer {
  fn buffer_kind_constant(&self) -> u32 {
    self.kind.buffer_kind_constant()
  }
}

pub enum RenderApiError {
  FailedToCreateBuffer,
  InvalidAttributeName(String),
}

impl RenderApiError {
  pub fn to_string(self) -> String {
    match self {
      RenderApiError::FailedToCreateBuffer => "Failed to create buffer".to_string(),
      RenderApiError::InvalidAttributeName(s) => format!("Invalid attribute name, {}", s),
    }
  }
}

impl<A> IntoAttributeValue for A where A: AttributeKey {
  fn with_context<C>(self, context: &C) -> Result<AttributeValue, RenderApiError> where C: RenderAPI {
    context.get_attribute(self)
  }
}

impl IntoAttributeValue for AttributeValue {
  fn with_context<C>(self, _: &C) -> Result<AttributeValue, RenderApiError> where C: RenderAPI {
    Ok(self)
  }
}
