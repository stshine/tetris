extern crate image;
extern crate pollster;
extern crate wgpu;
extern crate winit;

mod tetris;

use std::time::Instant;
use winit::{event::Event, event::WindowEvent, event_loop};
use tetris::Tetris;

const PX_PER_PIECE: f32 = 20.0;

fn main() {
    let width = PX_PER_PIECE * tetris::ROWS as f32;
    let height = PX_PER_PIECE * tetris::COLUMNS as f32;

    let event_loop = event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Tetris")
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .build(&event_loop)
        .unwrap();

    let mut game = pollster::block_on(Tetris::init(&window));
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
                    game.handle_key(input);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                game.render().unwrap();
            }
            _ => (),
        }
        if instant.elapsed().as_millis() >= 1000 {
            game.advance(tetris::Action::Down);
            instant = Instant::now();
        }
    });
}
