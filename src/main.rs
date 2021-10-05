extern crate gl;
extern crate glutin;
extern crate image;

mod gfx;
mod tetris;

use std::time::Instant;

use glutin::{event::Event, event::WindowEvent, event_loop};
use tetris::Tetris;

const PX_PER_PIECE: f32 = 20.0;

fn main() {
    let width = PX_PER_PIECE * tetris::ROWS as f32;
    let height = PX_PER_PIECE * tetris::COLUMNS as f32;

    let event_loop = event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_title("Tetris")
        .with_inner_size(glutin::dpi::LogicalSize::new(width, height));
    let window_context = glutin::ContextBuilder::new()
        .with_gl_profile(glutin::GlProfile::Core)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 5)))
        .build_windowed(window, &event_loop)
        .expect("Failed to create OpenGL context");
    let context = unsafe {
        window_context
            .make_current()
            .expect("Couldn't make window current")
    };

    ::gl::load_with(|s| context.get_proc_address(s) as *const std::ffi::c_void);

    let game = Tetris::init();

    let mut row: usize = 0;
    let mut column: usize = 0;

    let mut instant = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit;
                }
                WindowEvent::Resized(_) => {
                    // context.resize(*size);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key_code) = input.virtual_keycode {
                        match key_code {
                            glutin::event::VirtualKeyCode::Left => {
                                row = row.checked_sub(1).unwrap_or(0);
                            }
                            glutin::event::VirtualKeyCode::Right => {
                                row =  (row + 1).min(tetris::ROWS - 1);
                            }
                            glutin::event::VirtualKeyCode::Up => {
                                column = column.checked_sub(1).unwrap_or(0);
                            }
                            glutin::event::VirtualKeyCode::Down => {
                                column = (column + 1).min(tetris::COLUMNS - 1);
                            }
                            _ => {}
                        }
                    }
                    // match input.scancode {
                    //     0x4b => {
                    //     }
                    //     0x4d => {
                    //         row = row + 1;
                    //     }
                    //     0x50 => {
                    //         column = column - 1;
                    //     }
                    //     0x48 => {
                    //         column = column + 1;
                    //     }
                    //     _ => {}
                    // }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {}
            _ => (),
        }
        if instant.elapsed().as_millis() >= 1000 {
            column = (column + 1).min(tetris::COLUMNS - 1);
            instant = Instant::now();
        }
        game.draw_piece(row, column);
        context.swap_buffers().unwrap();
    });
}
