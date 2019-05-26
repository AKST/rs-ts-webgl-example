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
