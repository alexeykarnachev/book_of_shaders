use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::index::{IndicesSource, PrimitiveType};
use glium::vertex::EmptyVertexAttributes;
use glium::{Display, DrawParameters, Program, Surface};

use glium::{self, uniform};

const VERTEX_SHADER: &str = r#"
    #version 460

    const vec2 quadVertices[4] = {vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(-1.0, 1.0), vec2(1.0, 1.0)};
    void main()
    {
        gl_Position = vec4(quadVertices[gl_VertexID], 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 460

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

fn main() {
    let window_builder = WindowBuilder::new().with_title("Hello World");
    let context_builder = ContextBuilder::new();
    let event_loop = EventLoop::new();
    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

    let shader_program = Program::from_source(
        &display,
        VERTEX_SHADER,
        FRAGMENT_SHADER,
        None,
    )
    .unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => return,
            },
            _ => (),
        }

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame
            .draw(
                EmptyVertexAttributes { len: 4},
                IndicesSource::NoIndices {
                    primitives: PrimitiveType::TriangleStrip,
                },
                &shader_program,
                &uniform! {},
                &DrawParameters::default(),
            )
            .unwrap();

        frame.finish().unwrap();
    });
}
