use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460
    #define PI 3.14159265359

    uniform float u_time;
    uniform vec2 u_resolution;
    uniform vec2 u_cursor;

    out vec4 color;

    void main() {
        vec2 point = gl_FragCoord.xy / u_resolution;
        vec2 center = vec2(0.5, 0.8);

        vec2 pos = point - center;
        float angle = atan(pos.y, pos.x);
        float t = angle;
        float f = sin(t) * sqrt(abs(cos(t))) / (sin(t) + 7.0 / 5.0) - 2.0 * sin(t) + 2.0;

        color = vec4(step(1.0 / u_time * length(pos) / 0.18, vec3(f)), 1.0);
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
