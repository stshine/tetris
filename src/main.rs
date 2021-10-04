extern crate gl;
extern crate glutin;
extern crate image;

mod gfx;
mod tetris;

use glutin::{event_loop, event::Event, event::WindowEvent};
use tetris::Tetris;

const PX_PER_PIECE: f32 = 20.0;

fn main() {
    let width = PX_PER_PIECE * tetris::ROW as f32;
    let height = PX_PER_PIECE * tetris::COLUMN as f32;
    
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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit;
                }
                WindowEvent::Resized(_) => {
                    // context.resize(*size);
                }
                WindowEvent::KeyboardInput{input, ..} => {
                    println!("{}", input.scancode);
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                game.draw_piece(5, 10);
                context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
