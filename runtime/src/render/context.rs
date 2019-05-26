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
  fn with_context<C>(self, context: &C) -> Result<AttributeValue, C::Error> where C: RenderAPI;
}

pub trait RenderAPI {
  type Buffer: constants::HasBufferKind;
  type Error;

  fn create_buffer(
      &self,
      kind: constants::BufferKind,
  ) -> Result<Self::Buffer, Self::Error>;

  fn bind_buffer<V>(
      &self,
      buffer: Self::Buffer,
      view: &V,
      draw_kind: constants::DrawKind,
  ) where V: data::View;

  fn get_attribute<AK>(&self, key: AK) -> Result<AttributeValue, Self::Error> where AK: AttributeKey;
  fn vertex_attrib_pointer_with_i32<A>(&self, attr: A, size: i32, type_: u32, normalized: bool, stride: i32, offset: i32)
      -> Result<(), Self::Error> where A: IntoAttributeValue;

  fn enable_vertex_attrib_array<A>(&self, key: A) -> Result<(), Self::Error> where A: IntoAttributeValue;
}

pub struct GlProgram {
  gl: WebGlRenderingContext,
  program: WebGlProgram,
}

impl RenderAPI for GlProgram {
  type Buffer = data::WebRenderBuffer;
  type Error = GlProgrmError;

  fn create_buffer(
      &self,
      kind: constants::BufferKind,
  ) -> Result<Self::Buffer, Self::Error> {
    self.gl.create_buffer().ok_or(GlProgrmError::FailedToCreateBuffer).map(|internal| {
      data::WebRenderBuffer { kind, internal }
    })
  }

  fn bind_buffer<V>(
      &self,
      buffer: Self::Buffer,
      view: &V,
      draw_kind: constants::DrawKind,
  ) where V: data::View {
    use constants::{HasBufferKind, HasDrawKind};

    let kind = buffer.buffer_kind_constant();
    let draw = draw_kind.draw_kind_constant();
    self.gl.bind_buffer(kind, Some(&buffer.internal));
    self.gl.buffer_data_with_array_buffer_view(kind, view.object(), draw)
  }

  fn get_attribute<AK>(&self, key: AK) -> Result<AttributeValue, Self::Error> where AK: AttributeKey {
    let name = key.name();
    let glint = self.gl.get_attrib_location(&self.program, name);
    u32::try_from(glint).map_err(|_| GlProgrmError::InvalidAttributeName(name.to_string()))
  }

  fn vertex_attrib_pointer_with_i32<A>(&self, key: A, s: i32, t: u32, n: bool, st: i32, of: i32)
      -> Result<(), Self::Error> where A: IntoAttributeValue {
    key.with_context(self).map(|i| self.gl.vertex_attrib_pointer_with_i32(i, s, t, n, st, of))
  }

  fn enable_vertex_attrib_array<A>(&self, key: A) -> Result<(), Self::Error> where A: IntoAttributeValue {
    key.with_context(self).map(|i| self.gl.enable_vertex_attrib_array(i))
  }
}

pub enum GlProgrmError {
  FailedToCreateBuffer,
  InvalidAttributeName(String),
}

impl GlProgrmError {
  pub fn to_string(self) -> String {
    match self {
      GlProgrmError::FailedToCreateBuffer => "Failed to create buffer".to_string(),
      GlProgrmError::InvalidAttributeName(s) => format!("Invalid attribute name, {}", s),
    }
  }
}

impl<A> IntoAttributeValue for A where A: AttributeKey {
  fn with_context<C>(self, context: &C) -> Result<AttributeValue, C::Error> where C: RenderAPI {
    context.get_attribute(self)
  }
}

impl IntoAttributeValue for AttributeValue {
  fn with_context<C>(self, _: &C) -> Result<AttributeValue, C::Error> where C: RenderAPI {
    Ok(self)
  }
}

pub mod constants {
  use web_sys::{WebGlRenderingContext};

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
}

pub mod data {
  use wasm_bindgen::JsCast;
  use js_sys::{Object, Float32Array, WebAssembly};
  use web_sys::{WebGlBuffer};
  use super::constants::{BufferKind, HasBufferKind};

  pub struct Data<V: View, B: HasBufferKind> {
    pub buffer: B,
    pub view: V,
  }

  pub struct Float32View {
    size: usize,
    data: Float32Array,
  }

  pub struct WebRenderBuffer {
    pub kind: BufferKind,
    pub internal: WebGlBuffer,
  }

  pub trait View {
    fn length(&self) -> usize;
    fn object(&self) -> &Object;
  }

  impl HasBufferKind for WebRenderBuffer {
    fn buffer_kind_constant(&self) -> u32 {
      self.kind.buffer_kind_constant()
    }
  }

  impl Float32View {
    fn create(data_raw: &[f32]) -> Result<Self, DataViewError> {
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

  impl View for Float32View {
    fn length(&self) -> usize { self.size }
    fn object(&self) -> &Object { self.data.as_ref() }
  }

  pub enum DataViewError {
    FailedToCreateMemory,
  }
}
