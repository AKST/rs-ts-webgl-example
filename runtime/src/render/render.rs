use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::{
  WebGlBuffer,
  WebGlProgram,
  WebGlRenderingContext,
  WebGlShader,
};

#[derive(Debug)]
pub struct Render {
  buffer: WebGlBuffer,
  gl: WebGlRenderingContext,
  vert_shader: WebGlShader,
  frag_shader: WebGlShader,
  program: WebGlProgram,
  vertices: [f32; 9],
}

impl Render {
  pub fn create(
      gl: WebGlRenderingContext,
      program: WebGlProgram,
      frag_shader: WebGlShader,
      vert_shader: WebGlShader,
  ) -> Result<Self, RenderInitialisationError> {

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    let memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .map_err(|_| RenderInitialisationError::FailedToCreateMemory)?
      .buffer();

    let vertices_location = vertices.as_ptr() as u32 / 4;
    let vert_array = js_sys::Float32Array::new(&memory_buffer)
      .subarray(vertices_location, vertices_location + vertices.len() as u32);

    let buffer = gl
      .create_buffer()
      .ok_or(RenderInitialisationError::FailedToCreateBuffer)?;

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGlRenderingContext::STATIC_DRAW,
    );
    gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    Ok(Render {
      buffer,
      gl,
      vert_shader,
      frag_shader,
      program,
      vertices,
    })
  }

  pub fn draw(&self) {
    self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
    self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    self.gl.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (self.vertices.len() / 3) as i32,
    );
  }
}

pub enum RenderInitialisationError {
  FailedToCreateMemory,
  FailedToCreateBuffer,
}

impl RenderInitialisationError {
  pub fn to_string(self) -> String {
    match self {
      RenderInitialisationError::FailedToCreateMemory => "Failed to create memory".to_string(),
      RenderInitialisationError::FailedToCreateBuffer => "Failed to create buffer".to_string(),
    }
  }
}
