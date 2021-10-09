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

    let game = pollster::block_on(Tetris::init(&window));

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
                            winit::event::VirtualKeyCode::Left => {
                                row = row.checked_sub(1).unwrap_or(0);
                            }
                            winit::event::VirtualKeyCode::Right => {
                                row = (row + 1).min(tetris::ROWS - 1);
                            }
                            winit::event::VirtualKeyCode::Up => {
                                column = column.checked_sub(1).unwrap_or(0);
                            }
                            winit::event::VirtualKeyCode::Down => {
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
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                game.draw_piece(row, column).unwrap();
            }
            _ => (),
        }
        if instant.elapsed().as_millis() >= 1000 {
            column = (column + 1).min(tetris::COLUMNS - 1);
            instant = Instant::now();
        }
    });
}
