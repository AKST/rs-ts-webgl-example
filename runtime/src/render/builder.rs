use web_sys::{
  WebGlRenderingContext,
  WebGlShader,
};
use super::render::{Render, RenderInitialisationError};
use super::context::{WebRenderAPI, WebRenderBuffer};

pub type WebRender = Render<WebRenderAPI, WebRenderBuffer>;

#[derive(Debug)]
pub struct RenderBuilder {
  webgl_context: Option<WebGlRenderingContext>,
  vert_shader: Option<WebGlShader>,
  frag_shader: Option<WebGlShader>,
}

impl RenderBuilder {
  pub fn new() -> Self {
    RenderBuilder {
      webgl_context: None,
      vert_shader: None,
      frag_shader: None,
    }
  }

  pub fn set_context(&mut self, context: WebGlRenderingContext) {
    self.webgl_context = Some(context);
  }

  pub fn set_frag_shader(&mut self, shader_source: &str) -> Result<(), BuildError> {
    let shader_type = WebGlRenderingContext::FRAGMENT_SHADER;
    self.frag_shader = Some(self.create_shader(shader_source, shader_type)?);
    Ok(())
  }

  pub fn set_vert_shader(&mut self, shader_source: &str) -> Result<(), BuildError> {
    let shader_type = WebGlRenderingContext::VERTEX_SHADER;
    self.vert_shader = Some(self.create_shader(shader_source, shader_type)?);
    Ok(())
  }

  pub fn build_render(&self) -> Result<WebRender, BuildError> {
    let context = self.webgl_context.clone().ok_or(BuildError::ExpectedContext)?;
    let vert_shader = self.vert_shader.clone().ok_or(BuildError::ExpectedVertShaded)?;
    let frag_shader = self.frag_shader.clone().ok_or(BuildError::ExpectedFragShaded)?;
    let program = context.create_program().ok_or(BuildError::CannotCreateProgram)?;

    context.attach_shader(&program, &vert_shader);
    context.attach_shader(&program, &frag_shader);
    context.link_program(&program);

    let did_link = context
      .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
      .as_bool()
      .ok_or(BuildError::FailedToLinkProgram)?;

    return if did_link {
      context.use_program(Some(&program));
      Ok(Render::create(WebRenderAPI::create(context, program))?)
    } else {
      Err(BuildError::FailedToLinkProgram)
    };
  }

  fn get_context(&self) -> Option<&WebGlRenderingContext> {
    match &self.webgl_context {
      Some(ref value) => Some(&value),
      None => None,
    }
  }

  fn create_shader(&self, shader_source: &str, shader_type: u32) -> Result<WebGlShader, BuildError> {
    let context = self.get_context().ok_or(BuildError::ExpectedContext)?;

    let shader = context.create_shader(shader_type).ok_or(BuildError::CannotCreateShader)?;
    context.shader_source(&shader, shader_source);
    context.compile_shader(&shader);

    let did_compile = context
      .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
      .as_bool()
      .ok_or(BuildError::FailedToCompileShader(None))?;

    return if did_compile {
      Ok(shader)
    } else {
      Err(BuildError::FailedToCompileShader(context.get_shader_info_log(&shader)))
    }
  }
}

pub enum BuildError {
  ExpectedContext,
  ExpectedVertShaded,
  ExpectedFragShaded,
  FailedToCompileShader(Option<String>),
  FailedToLinkProgram,
  CannotCreateShader,
  CannotCreateProgram,
  InitialisationError(RenderInitialisationError),
}

impl From<RenderInitialisationError> for BuildError {
  fn from(error: RenderInitialisationError) -> Self {
    BuildError::InitialisationError(error)
  }
}

impl BuildError {
  pub fn to_string(self) -> String {
    match self {
      BuildError::ExpectedContext => "expected webgl context to be defined".to_string(),
      BuildError::ExpectedFragShaded => "expected frag shader to be defined".to_string(),
      BuildError::ExpectedVertShaded => "expected vert shader to be defined".to_string(),
      BuildError::FailedToCompileShader(reason) => match reason {
        None => "failed to compile shader, for an unknown reason".to_string(),
        Some(reason) => format!("failed to compile shader: {}", reason),
      },
      BuildError::FailedToLinkProgram => "failed to link program".to_string(),
      BuildError::CannotCreateShader => "could not create a shader from the context".to_string(),
      BuildError::CannotCreateProgram => "could not create a program from the context".to_string(),
      BuildError::InitialisationError(error) => {
        format!("failed to initialize render: {}", error.to_string())
      },
    }
  }
}
