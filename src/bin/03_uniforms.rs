use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460

    uniform float u_time;
    uniform vec2 u_resolution;
    uniform vec2 u_cursor;

    out vec4 color;

    float get_dist_to_cursor() {
        float x = gl_FragCoord.x;
        float y = gl_FragCoord.y;
        float max_dist = sqrt(pow(u_resolution.x, 2.0) + pow(u_resolution.y, 2.0));
        float curr_dist = sqrt(pow(u_cursor.x - x, 2.0) + pow(u_cursor.y - y, 2.0)); 
        return curr_dist / max_dist;
    }

    vec4 get_color() {
        float dist = get_dist_to_cursor();
        color = vec4(
            pow((1.0 - dist), 30) * gl_FragCoord.x / u_resolution.x,
            pow((1.0 - dist), 30) * gl_FragCoord.y / u_resolution.y,
            pow((1.0 - dist), 30) * abs(cos(u_time)), 
            1.0
        );

        return color;
    }

    void main() {
        color = get_color();
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
