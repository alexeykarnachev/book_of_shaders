use book_of_shaders::{RenderState, Renderer};
use glium::{
    uniform,
    uniforms::{EmptyUniforms, UniformsStorage},
};

const FRAGMENT_SHADER: &str = r#"
    #version 460

    uniform float u_time;
    uniform vec2 u_display_size;
    uniform vec2 u_cursor_coords;

    out vec4 color;

    float get_dist_to_cursor() {
        float frag_x = gl_FragCoord.x;
        float frag_y = u_display_size.y - gl_FragCoord.y;
        float max_dist = sqrt(pow(u_display_size.x, 2.0) + pow(u_display_size.y, 2.0));
        float curr_dist = sqrt(pow(u_cursor_coords.x - frag_x, 2.0) + pow(u_cursor_coords.y - frag_y, 2.0)); 
        return curr_dist / max_dist;
    }

    vec4 get_color() {
        float dist = get_dist_to_cursor();
        color = vec4(
            pow((1.0 - dist), 30) * gl_FragCoord.x / u_display_size.x,
            pow((1.0 - dist), 30) * gl_FragCoord.y / u_display_size.y,
            pow((1.0 - dist), 30) * abs(cos(u_time)), 
            1.0
        );

        return color;
    }

    void main() {
        color = get_color();
    }
"#;

type U = UniformsStorage<
    'static,
    (f32, f32),
    UniformsStorage<'static, (f32, f32), UniformsStorage<'static, f32, EmptyUniforms>>,
>;

fn uniforms_f(render_state: RenderState) -> U {
    uniform! {
        u_time: render_state.passed_time,
        u_display_size: render_state.display_size,
        u_cursor_coords: render_state.cursor_coords,
    }
}

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw(&uniforms_f);
}
