use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460

    uniform float u_time;
    uniform vec2 u_resolution;

    float PI = 3.14159265359;
    vec3 LINE_COLOR_COLD = vec3(0.0, 0.0, 1.0);
    vec3 LINE_COLOR_HOT = vec3(1.0, 0.0, 0.0);
    vec3 BACK_COLOR_COLD = vec3(0.2, 0.2, 0.6);
    vec3 BACK_COLOR_HOT = vec3(0.6, 0.2, 0.2);


    float wave(float x, float ampl, float freq, float h_phase, float v_phase) {
        return ampl * sin((x - h_phase) * freq * PI) + v_phase;
    }

    out vec4 color;

    void main() {
        vec2 coords = gl_FragCoord.xy / u_resolution;
        float x = coords.x;

        float y = 0.5 * wave(x, 0.3, 8.0, u_time, 0.5) + 0.5 * wave(x, 0.7, 0.1, u_time, 0.5);

        float c = step(abs(coords.y - y), 0.005);

        vec3 line_color = y * LINE_COLOR_HOT + (1.0 - y) * LINE_COLOR_COLD;
        vec3 back_color = y * BACK_COLOR_HOT + (1.0 - y) * BACK_COLOR_COLD;
        
        color = vec4(c * line_color + (1.0 - c) * back_color, 1.0);
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
