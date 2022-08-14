use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460
    #define PI 3.14159265359

    uniform float u_time;
    uniform vec2 u_resolution;
    uniform vec2 u_cursor;

    out vec4 color;

    vec2 tile_space(vec2 p, float n_rows, float n_cols) {
        mat2 tiling = mat2(
            n_cols, 0.0,
            0.0, n_rows
        );
        return fract(tiling * p);
    }

    vec2 shift_space(vec2 p, float h_shift, float v_shift) {
        mat3 shifting = mat3(
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            h_shift, v_shift, 1.0
        );
        return (shifting * vec3(p, 1.0)).xy;
    }

    vec2 get_tile_idx(vec2 p, float n_rows, float n_cols) {
        return vec2(int(p.x * n_cols), int(p.y * n_rows));
    }

    float rect(vec2 p) {
        return 1.0 - step(min(min(p.x, p.y), min(1.0 - p.x, 1.0 - p.y)), 0.1);
    }

    void main() {
        vec2 p = gl_FragCoord.xy / u_resolution;
        float n_rows = 10.0;
        float n_cols = 7.0;

        vec2 tile_idx = get_tile_idx(p, n_rows, n_cols);

        float period = 2.0;
        float t = u_time - period * 2 * int(u_time / (period * 2));
        if (t > period && int(tile_idx.y) % 2 == 0) {
            t -= period;
            p = shift_space(p, t / period, 0.0);
        } else if (t <= period && int(tile_idx.x) % 2 == 0) {
            p = shift_space(p, 0.0, t / period);
        }

        p = tile_space(p, n_rows, n_cols);

        float r = rect(p);
        color = vec4(vec3(r), 1.0);
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
