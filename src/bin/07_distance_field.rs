use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460

    uniform float u_time;
    uniform vec2 u_resolution;
    uniform vec2 u_cursor;

    out vec4 color;

    void main() {

        vec2 center = u_cursor / u_resolution;
        vec2 point = gl_FragCoord.xy / u_resolution;
        float dist_from_center = length(point - center);
        float line_thickness = 0.002;

        color = vec4(1.0);
        int n_levels = 50;
        float cur_level = 0.0;
        float level_step = 1.5 / float(n_levels);

        for (int i = 0; i < n_levels; ++i) {
            if (abs(dist_from_center - cur_level) <= line_thickness) {
                color = vec4(vec3(dist_from_center), 1.0);
            }
            cur_level += level_step;
        }
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
