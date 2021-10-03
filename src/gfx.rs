#[allow(dead_code)]
use gl::types::{GLchar, GLenum, GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use std::ffi::CString;
use std::ptr;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ShaderKind {
    Vertex,
    Fragment,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BufferTarget {
    Vertex,
    Index,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Clone, Debug)]
pub struct GLBuffer {
    pub gl_buffer: GLuint,
}

impl Drop for GLBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.gl_buffer);
        }
    }
}

pub fn create_buffer() -> GLBuffer {
    let mut gl_buffer = 0;
    unsafe {
        gl::GenBuffers(1, &mut gl_buffer);
    }
    GLBuffer { gl_buffer }
}

pub struct GLProgram {
    pub gl_program: GLuint,
    vertex_shader: GLShader,
    fragment_shader: GLShader,
}

impl Drop for GLProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.gl_program);
        }
    }
}

pub struct GLShader {
    gl_shader: GLuint,
}

impl Drop for GLShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.gl_shader);
        }
    }
}

fn create_shader(name: &str, source: &str, kind: ShaderKind) -> GLShader {
    let gl_shader_kind = match kind {
        ShaderKind::Vertex => gl::VERTEX_SHADER,
        ShaderKind::Fragment => gl::FRAGMENT_SHADER,
    };

    unsafe {
        let gl_shader = gl::CreateShader(gl_shader_kind);
        gl::ShaderSource(
            gl_shader,
            1,
            [source.as_ptr() as *const GLchar].as_ptr(),
            [source.len()].as_ptr() as *const GLint,
        );
        gl::CompileShader(gl_shader);

        let mut compile_status = 0;
        gl::GetShaderiv(gl_shader, gl::COMPILE_STATUS, &mut compile_status);
        if compile_status != gl::TRUE as GLint {
            let mut info_log_length = 0;
            gl::GetShaderiv(gl_shader, gl::INFO_LOG_LENGTH, &mut info_log_length);
            let mut info_log = vec![0; info_log_length as usize];
            gl::GetShaderInfoLog(
                gl_shader,
                info_log.len() as GLint,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            // error!("Shader info log:\n{}", String::from_utf8_lossy(&info_log));
            panic!("{:?} shader '{}' compilation failed", kind, name);
        }
        return GLShader { gl_shader };
    }
}

pub fn create_program(name: &str, vertex_shader: &str, fragment_shader: &str) -> GLProgram {
    let gl_program;
    let vertex_shader = create_shader(name, vertex_shader, ShaderKind::Vertex);
    let fragment_shader = create_shader(name, fragment_shader, ShaderKind::Fragment);
    unsafe {
        gl_program = gl::CreateProgram();
        gl::AttachShader(gl_program, vertex_shader.gl_shader);
        gl::AttachShader(gl_program, fragment_shader.gl_shader);
        gl::LinkProgram(gl_program);

        let mut link_status = 0;
        gl::GetProgramiv(gl_program, gl::LINK_STATUS, &mut link_status);
        if link_status != gl::TRUE as GLint {
            let mut info_log_length = 0;
            gl::GetProgramiv(gl_program, gl::INFO_LOG_LENGTH, &mut info_log_length);
            let mut info_log = vec![0; info_log_length as usize];
            gl::GetProgramInfoLog(
                gl_program,
                info_log.len() as GLint,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            eprintln!("Program info log:\n{}", String::from_utf8_lossy(&info_log));
            panic!("Program '{}' linking failed", name);
        }
    }

    GLProgram {
        gl_program,
        vertex_shader,
        fragment_shader,
    }
}
