use gl::types::{GLchar, GLint, GLsizeiptr, GLuint, GLvoid};

use crate::gfx::{GLBuffer, GLProgram, create_buffer, create_program};

pub const ROW: usize = 10;
pub const COLUMN: usize = 20;

const VERTICES: [[i32; 2]; 4] = [
    [0, 0], [1, 0], [0, 1], [1, 1], 
];

pub struct Tetris {
    program: GLProgram,
    piece_vao: GLuint,
    piece_vertices: GLBuffer,
    row: GLint,
    column: GLint
}

impl Tetris {
    pub fn init() -> Self {
        let program = create_program(
            "tetris", 
            include_str!("../resources/tetris.vs.glsl"), 
            include_str!("../resources/tetris.fs.glsl")
        );

        let mut piece_vao = 0;
        let piece_vertices = create_buffer();
        let row;
        let column;
        unsafe {
            // When passing string to C API we must make sure it ends with zero.
            row = gl::GetUniformLocation(program.gl_program, b"row\0".as_ptr() as *const GLchar);
            column = gl::GetUniformLocation(program.gl_program, b"column\0".as_ptr() as *const GLchar);
            gl::GenVertexArrays(1, &mut piece_vao);
            gl::BindVertexArray(piece_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, piece_vertices.gl_buffer);
            gl::BufferData(gl::ARRAY_BUFFER,
                (VERTICES.len() * std::mem::size_of::<i32>() * 2) as GLsizeiptr,
                VERTICES.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );
            gl::VertexAttribPointer(
                0,
                2,
                gl::INT,
                gl::FALSE,
                (2 * std::mem::size_of::<i32>()) as GLint,
                0 as *const GLvoid
            );
            gl::EnableVertexAttribArray(0);
        }

        Self {
            program,
            piece_vao,
            piece_vertices,
            row,
            column
        }
    }

    pub fn draw_piece(&self, row: usize, column: usize) {
        unsafe {
            gl::UseProgram(self.program.gl_program);
            gl::BindVertexArray(self.piece_vao);

            gl::Uniform1i(self.row, row as GLint);
            gl::Uniform1i(self.column, column as GLint);

            gl::ClearColor(0.0,0.0,0.0,0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }
}
