use wasm_bindgen::JsCast;
use js_sys::{Object, Float32Array, WebAssembly};
use super::constants::{HasBufferKind, ViewPrecision, HasViewPrecision};

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

pub trait View: HasViewPrecision {
  fn length(&self) -> usize;
  fn object(&self) -> &Object;
  fn get_precision(&self) -> ViewPrecision;
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
