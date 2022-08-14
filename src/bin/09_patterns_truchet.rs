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

    vec2 rotate_space(vec2 p, float angle) {
        p -= 0.5;
        mat2 rotation = mat2(
            cos(angle), -sin(angle),
            sin(angle), cos(angle)
        );
        p = rotation * p;
        p += 0.5;
        return p;
    }

    float triangle(vec2 p) {
        return step(p.x - p.y, 0.001);
    }

    int get_tile_idx(vec2 p, float n_rows, float n_cols) {
        vec2 idx = vec2(int(p.x * n_cols), int(p.y * n_rows));
        return int(idx.y * n_cols + idx.x);
    }

    void main() {
        vec2 p = gl_FragCoord.xy / u_resolution;

        float n_rows = 10;
        float n_cols = 10;

        p = tile_space(p, n_rows, n_cols);

        float tile_idx = get_tile_idx(p, 2.0, 2.0);
        if (tile_idx == 0) {
            p = rotate_space(p, cos(u_time * PI));
        } else if (tile_idx == 1) {
            p = rotate_space(p, sin(u_time * PI));
        } else if (tile_idx == 2) {
            p = rotate_space(p, cos(u_time * PI));
        } else if (tile_idx == 3) {
            p = rotate_space(p, sin(u_time * PI));
        }
        p = tile_space(p, 2.0, 2.0);

        color = vec4(vec3(triangle(p)), 1.0);
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
