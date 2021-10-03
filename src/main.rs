extern crate gl;
extern crate glutin;
extern crate image;

mod gfx;
use glutin::{event_loop, event::Event, event::WindowEvent};

fn main() {
    let event_loop = event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_title("Tetris")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024., 768.));
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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    context.resize(*size);
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // pipeline.clear(clear_color);
                //                    pipeline.draw_arrays(3, Primitive::Triangle);
                // pipeline.draw_elements(&index_buffer, 6, Primitive::Triangle);
                context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
