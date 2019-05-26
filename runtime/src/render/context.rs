use std::convert::TryFrom;
use web_sys::{
  WebGlProgram,
  WebGlRenderingContext,
};

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
  type Buffer: constants::HasBufferKind;

  fn create_buffer(
      &self,
      kind: constants::BufferKind,
  ) -> Result<Self::Buffer, RenderApiError>;

  fn bind_buffer<V>(
      &self,
      buffer: &Self::Buffer,
      view: &V,
      draw_kind: constants::DrawKind,
  ) where V: data::View;

  fn get_attribute<AK>(&self, key: AK) -> Result<AttributeValue, RenderApiError>
      where AK: AttributeKey;

  fn vertex_attrib_pointer_with_i32<A>(
      &self,
      key: A,
      size: i32,
      precision: constants::ViewPrecision,
      normalized: bool,
      stride: i32,
      offset: i32
  ) -> Result<(), RenderApiError> where A: IntoAttributeValue;

  fn enable_vertex_attrib_array<A>(&self, key: A) -> Result<(), RenderApiError>
      where A: IntoAttributeValue;

  fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32);

  fn clear(&self, mask: constants::ClearMask);

  fn draw_arrays(&self, mode: constants::DrawArrayKind, first: i32, count: i32);
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
  type Buffer = data::WebRenderBuffer;

  fn create_buffer(
      &self,
      kind: constants::BufferKind,
  ) -> Result<Self::Buffer, RenderApiError> {
    self.gl.create_buffer().ok_or(RenderApiError::FailedToCreateBuffer).map(|internal| {
      data::WebRenderBuffer { kind, internal }
    })
  }

  fn bind_buffer<V>(
      &self,
      buffer: &Self::Buffer,
      view: &V,
      draw_kind: constants::DrawKind,
  ) where V: data::View {
    use constants::{HasBufferKind, HasDrawKind};

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
      precision: constants::ViewPrecision,
      normalized: bool,
      stride: i32,
      offset: i32
  ) -> Result<(), RenderApiError> where A: IntoAttributeValue {
    use constants::HasViewPrecision;

    key.with_context(self).map(|index| {
      let precision_type = precision.view_precision_constant();
      self.gl.vertex_attrib_pointer_with_i32(
          index,
          size,
          precision_type,
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

  fn clear(&self, mask: constants::ClearMask) {
    use constants::HasClearMaskKind;
    self.gl.clear(mask.clear_mask_constant());
  }

  fn draw_arrays(&self, mode: constants::DrawArrayKind, first: i32, count: i32) {
    use constants::HasDrawArrayKind;
    self.gl.draw_arrays(mode.draw_array_kind_constant(), first, count);
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

pub mod constants {
  use web_sys::{WebGlRenderingContext};

  #[derive(Clone, Copy, Debug)]
  pub enum DrawKind {
    StaticDraw,
    DynamicDraw,
    StreamDraw,
  }

  pub trait HasDrawKind {
    fn draw_kind_constant(&self) -> u32;
  }

  impl HasDrawKind for DrawKind {
    fn draw_kind_constant(&self) -> u32 {
      match self {
        DrawKind::StaticDraw => WebGlRenderingContext::STATIC_DRAW,
        DrawKind::DynamicDraw => WebGlRenderingContext::DYNAMIC_DRAW,
        DrawKind::StreamDraw => WebGlRenderingContext::STREAM_DRAW,
      }
    }
  }

  #[derive(Clone, Copy, Debug)]
  pub enum DrawArrayKind {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    TriangleStrip,
    TriangleFan,
    Triangles,
  }

  pub trait HasDrawArrayKind {
    fn draw_array_kind_constant(&self) -> u32;
  }

  impl HasDrawArrayKind for DrawArrayKind {
    fn draw_array_kind_constant(&self) -> u32 {
      match self {
        DrawArrayKind::Points => WebGlRenderingContext::POINTS,
        DrawArrayKind::LineStrip => WebGlRenderingContext::LINE_STRIP,
        DrawArrayKind::LineLoop => WebGlRenderingContext::LINE_LOOP,
        DrawArrayKind::Lines => WebGlRenderingContext::LINES,
        DrawArrayKind::TriangleStrip => WebGlRenderingContext::TRIANGLE_STRIP,
        DrawArrayKind::TriangleFan => WebGlRenderingContext::TRIANGLE_FAN,
        DrawArrayKind::Triangles => WebGlRenderingContext::TRIANGLES,
      }
    }
  }

  #[derive(Clone, Copy, Debug)]
  pub enum ClearMask {
    ColorBufferBit,
    DepthBufferBit,
    StencilBufferBit,
  }

  pub trait HasClearMaskKind {
    fn clear_mask_constant(&self) -> u32;
  }

  impl HasClearMaskKind for ClearMask {
    fn clear_mask_constant(&self) -> u32 {
      match self {
        ClearMask::ColorBufferBit => WebGlRenderingContext::COLOR_BUFFER_BIT,
        ClearMask::DepthBufferBit => WebGlRenderingContext::DEPTH_BUFFER_BIT,
        ClearMask::StencilBufferBit => WebGlRenderingContext::STENCIL_BUFFER_BIT,
      }
    }
  }

  #[derive(Clone, Copy, Debug)]
  pub enum BufferKind {
    ArrayBuffer,
    ElementBuffer,
  }

  pub trait HasBufferKind {
    fn buffer_kind_constant(&self) -> u32;
  }

  impl HasBufferKind for BufferKind {
    fn buffer_kind_constant(&self) -> u32 {
      match self {
        BufferKind::ArrayBuffer => WebGlRenderingContext::ARRAY_BUFFER,
        BufferKind::ElementBuffer => WebGlRenderingContext::ELEMENT_ARRAY_BUFFER_BINDING,
      }
    }
  }

  #[derive(Clone, Copy, Debug)]
  pub enum ViewPrecision {
    Byte,
    Short,
    UnsignedByte,
    UnsignedShort,
    Float,
  }

  pub trait HasViewPrecision {
    fn view_precision_constant(&self) -> u32;
  }

  impl HasViewPrecision for ViewPrecision {
    fn view_precision_constant(&self) -> u32 {
      match self {
        ViewPrecision::Byte => WebGlRenderingContext::BYTE,
        ViewPrecision::Short => WebGlRenderingContext::SHORT,
        ViewPrecision::UnsignedByte => WebGlRenderingContext::UNSIGNED_BYTE,
        ViewPrecision::UnsignedShort => WebGlRenderingContext::UNSIGNED_SHORT,
        ViewPrecision::Float => WebGlRenderingContext::FLOAT,
      }
    }
  }
}

pub mod data {
  use wasm_bindgen::JsCast;
  use js_sys::{Object, Float32Array, WebAssembly};
  use web_sys::{WebGlBuffer};
  use super::constants::{BufferKind, HasBufferKind, ViewPrecision, HasViewPrecision};

  #[derive(Clone, Copy, Debug)]
  pub struct Data<V: View, B: HasBufferKind> {
    pub buffer: B,
    pub view: V,
  }

  #[derive(Clone, Debug)]
  pub struct Float32View {
    size: usize,
    data: Float32Array,
  }

  #[derive(Clone, Debug)]
  pub struct WebRenderBuffer {
    pub kind: BufferKind,
    pub internal: WebGlBuffer,
  }

  pub trait View: HasViewPrecision {
    fn length(&self) -> usize;
    fn object(&self) -> &Object;
    fn get_precision(&self) -> ViewPrecision;
  }

  impl HasBufferKind for WebRenderBuffer {
    fn buffer_kind_constant(&self) -> u32 {
      self.kind.buffer_kind_constant()
    }
  }

  impl Float32View {
    pub fn create(data_raw: &[f32]) -> Result<Self, DataViewError> {
      let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .map_err(|_| DataViewError::FailedToCreateMemory)?
        .buffer();

      let data_location = data_raw.as_ptr() as u32 / 4;
      let data = js_sys::Float32Array::new(&memory_buffer)
        .subarray(data_location, data_location + data_raw.len() as u32);

      Ok(Float32View { data, size: data_raw.len() })
    }
  }

  impl HasViewPrecision for Float32View {
    fn view_precision_constant(&self) -> u32 {
      self.get_precision().view_precision_constant()
    }
  }

  impl View for Float32View {
    fn length(&self) -> usize { self.size }
    fn object(&self) -> &Object { self.data.as_ref() }

    fn get_precision(&self) -> ViewPrecision {
      ViewPrecision::Float
    }
  }

  #[derive(Clone, Copy)]
  pub enum DataViewError {
    FailedToCreateMemory,
  }

  impl DataViewError {
    pub fn to_string(&self) -> String {
      match self {
        DataViewError::FailedToCreateMemory => "Failed to create memory".to_string(),
      }
    }
  }
}
