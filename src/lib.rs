use std::time::Instant;

use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{IndicesSource, PrimitiveType};
use glium::uniforms::Uniforms;
use glium::vertex::EmptyVertexAttributes;
use glium::{Display, DrawParameters, Program, Surface};

pub struct RenderState {
    pub cursor_coords: (f32, f32),
    pub display_size: (f32, f32),
    pub passed_time: f32,
}

pub struct Renderer {
    display: Display,
    shader_program: Program,
    event_loop: EventLoop<()>,
}

impl Renderer {
    pub fn from_fragment_shader(fragment_shader: &str) -> Renderer {
        let window_builder = WindowBuilder::new().with_title("Fragment");
        let context_builder = ContextBuilder::new();
        let event_loop = EventLoop::new();
        let display = Display::new(window_builder, context_builder, &event_loop).unwrap();
        let vertex_shader: &str = r#"
        #version 460

        const vec2 quadVertices[4] = {vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(-1.0, 1.0), vec2(1.0, 1.0)};
        void main()
        {
            gl_Position = vec4(quadVertices[gl_VertexID], 0.0, 1.0);
        }
        "#;
        let shader_program =
            Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

        Renderer {
            display,
            shader_program,
            event_loop,
        }
    }

    pub fn draw<U: Uniforms>(self, uniforms_f: &'static dyn Fn(RenderState) -> U) {
        let start_time = Instant::now();
        let mut cursor_coords = (0.0f32, 0.0f32);

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_coords = (position.x as f32, position.y as f32);
                    }
                    _ => (),
                },
                _ => (),
            }

            let (width, height) = self.display.get_framebuffer_dimensions();
            let display_size = (width as f32, height as f32);
            let passed_time = (Instant::now() - start_time).as_micros() as f32 / 1_000_000f32;

            let mut frame = self.display.draw();
            frame.clear_color(0.0, 0.0, 0.0, 1.0);
            let uniforms = uniforms_f(RenderState {
                cursor_coords,
                display_size,
                passed_time,
            });
            frame
                .draw(
                    EmptyVertexAttributes { len: 4 },
                    IndicesSource::NoIndices {
                        primitives: PrimitiveType::TriangleStrip,
                    },
                    &self.shader_program,
                    &uniforms,
                    &DrawParameters::default(),
                )
                .unwrap();

            frame.finish().unwrap();
        });
    }
}
