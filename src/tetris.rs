use wgpu::{VertexBufferLayout, util::DeviceExt};

pub const ROWS: usize = 10;
pub const COLUMNS: usize = 20;
const VERTICES: [[u32; 2]; 4] = [[0, 0], [0, 1], [1, 0], [1, 1]];

pub fn cast_slice<T>(data: &[T]) -> &[u8] {
    use std::{mem::size_of, slice::from_raw_parts};

    unsafe { from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

enum Tetromino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

impl Tetromino {
    pub fn color(&self) -> u32 {
        match self {
            Self::I => 0x2DD4BF,
            Self::O => 0xFACC15,
            Self::T => 0xC084FC,
            Self::J => 0x60A5FA,
            Self::L => 0xFB923C,
            Self::S => 0x4ADE80,
            Self::Z => 0xF87171,
        }
    }

    pub fn blocks(&self) -> [Block; 4] {
        match self {
            Self::I => [
                Block::new(-2, -1),
                Block::new(-1, -1),
                Block::new(0, -1),
                Block::new(1, -1),
            ],
            Self::O => [
                Block::new(0, 0),
                Block::new(1, 0),
                Block::new(0, 1),
                Block::new(1, 1),
            ],
            Self::T => [
                Block::new(0, 0),
                Block::new(1, 0),
                Block::new(0, 1),
                Block::new(-1, 0),
            ],
            Self::J => [
                Block::new(0, 0),
                Block::new(1, 0),
                Block::new(-1, 1),
                Block::new(-1, 0),
            ],
            Self::L => [
                Block::new(0, 0),
                Block::new(1, 0),
                Block::new(1, 1),
                Block::new(-1, 0),
            ],
            Self::S => [
                Block::new(0, 0),
                Block::new(0, 1),
                Block::new(1, 1),
                Block::new(-1, 0),
            ],
            Self::Z => [
                Block::new(0, 0),
                Block::new(1, 0),
                Block::new(0, 1),
                Block::new(-1, 1),
            ],
        }
    }
}

pub struct Tetris {
    surface: wgpu::Surface,
    // adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    // piece_shader: wgpu::ShaderModule,
    piece_vertices: wgpu::Buffer,
    matrix_buffer: wgpu::Buffer,
    // piece_index: wgpu::Buffer,
    // index_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,

    matrix: [[u32; ROWS]; COLUMNS],
    row: usize,
    column: usize,
}

impl Tetris {
    pub async fn init(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("../resources/tetris.wgsl").into()),
        });

        let matrix = <[[u32; ROWS]; COLUMNS]>::default();

        let matrix_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Matrix buffer"),
            contents: cast_slice(&matrix),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let matrix_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<u32>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint32,
                }
            ]
        };

        // let piece_index = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Index Buffer"),
        //     contents: unsafe { &std::mem::transmute::<[u32; 2], [u8; 8]>([1, 1]) },
        //     usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        // });

        // let index_bind_group_layout =
        //     device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //         entries: &[wgpu::BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: wgpu::ShaderStages::VERTEX,
        //             ty: wgpu::BindingType::Buffer {
        //                 ty: wgpu::BufferBindingType::Uniform,
        //                 has_dynamic_offset: false,
        //                 min_binding_size: None,
        //             },
        //             count: None,
        //         }],
        //         label: Some("index_binding_group_layout"),
        //     });

        // let index_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: &index_bind_group_layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: piece_index.as_entire_binding(),
        //     }],
        //     label: Some("index_binding_group"),
        // });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Piece pipeline"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let piece_vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("piece vertices"),
            contents: unsafe { &std::mem::transmute::<[[u32; 2]; 4], [u8; 32]>(VERTICES) },
            usage: wgpu::BufferUsages::VERTEX,
        });

        let buffer_layout = wgpu::VertexBufferLayout {
            array_stride: (std::mem::size_of::<u32>() * 2) as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Uint32x2,
            }],
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Piece pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[buffer_layout, matrix_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                // Note: As a 2D application we should not set cull mode.
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        Self {
            surface,
            device,
            queue,
            config,
            piece_vertices,
            matrix_buffer,
            // piece_index,
            // index_bind_group,
            render_pipeline,
            matrix,
            row: 0,
            column: 0,
        }
    }

    fn can_advance(&self) -> bool {
        self.column < COLUMNS - 1 && self.matrix[self.column+1][self.row] == 0
    }

    pub fn advance(&mut self) {
        if self.can_advance() {
            self.column = self.column + 1;
        } else {
            self.matrix[self.column][self.row] = 1;
            for column in 0..COLUMNS {
                if self.matrix[column].iter().all(|x| *x != 0) {
                    for col in (0..column).rev() {
                        for row in 0..ROWS {
                            self.matrix[col+1][row] = self.matrix[col][row];
                            self.matrix[col][row] = 0; 
                        }
                    }
                }
            }
            self.row = 0;
            self.column = 0;
        }
        self.update();
    }

    pub fn handle_key(&mut self, input: &winit::event::KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            match key_code {
                winit::event::VirtualKeyCode::Left if self.row > 0 => {
                    self.row = self.row - 1;
                }
                winit::event::VirtualKeyCode::Right if self.row < ROWS - 1 => {
                    self.row = self.row + 1;
                }
                winit::event::VirtualKeyCode::Down if self.column < COLUMNS - 1 => {
                    self.advance();
                }
                winit::event::VirtualKeyCode::Up => {
                    // self.column = self.column - 1;
                }
                _ => {}
            }
        }
        self.update();
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

    fn update(&self) {
        // Instead of save the current tetromino in matrix directly, we use a copy.
        let mut matrix = self.matrix.clone();
        matrix[self.column][self.row] = 1;
        self.queue.write_buffer(&self.matrix_buffer, 0, cast_slice(&matrix));
    }

    fn draw_matrix(&self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());


        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            // render_pass.set_bind_group(0, &self.index_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.piece_vertices.slice(..));
            render_pass.set_vertex_buffer(1, self.matrix_buffer.slice(..));
            render_pass.draw(0..4, 0..(ROWS*COLUMNS) as u32);
        }
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        return self.draw_matrix();
    }
}
