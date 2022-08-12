use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460
    #define PI 3.14159265359

    uniform float u_time;
    uniform vec2 u_resolution;
    uniform vec2 u_cursor;

    out vec4 color;

    float circle(vec2 p, float r) {
        p -= 0.5;
        float thickness = 0.01;
        return step(abs(length(p) - r), thickness);
    }

    float cross(vec2 p, float r) {
        float thickness = 0.01;
        float len = length(p - 0.5);
        return min(
            1.0, 
            (step(abs(p.x - p.y), thickness) + step(abs(1.0 - (p.x + p.y)), thickness)) * step(len, r)
        );
    }

    vec2 tile_space(float n_rows, float n_cols) {
        vec2 p = gl_FragCoord.xy / u_resolution;
        p.x = fract(p.x * n_cols);
        p.y = fract(p.y * n_rows);
        return p;
    }

    vec2 get_tile_index(float n_rows, float n_cols) {
        vec2 p = gl_FragCoord.xy / u_resolution;
        return vec2(int(p.x * n_cols), int(p.y * n_rows));
    }

    void main() {
        float grid_size = 20.0;
        float n_rows = grid_size;
        float n_cols = grid_size;
        vec2 p = tile_space(n_rows, n_cols);

        vec2 tile_index = get_tile_index(n_rows, n_cols);
        float c = 0.0;
        if (abs(tile_index.x - tile_index.y) == int(u_time) % int(n_rows)) {
            c = cross(p, 0.4);
        } else {
            c = circle(p, 0.4);
        }
        color = vec4(vec3(c), 1.0);
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
