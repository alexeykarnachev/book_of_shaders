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

    vec2 scale_space(vec2 p, float s) {
        p -= 0.5;
        mat2 scaling = mat2(
            1.0 / s, 0.0,
            0.0, 1.0 / s
        );
        return scaling * p + 0.5;
    }

    vec2 get_tile_index2d(float n_rows, float n_cols) {
        vec2 p = gl_FragCoord.xy / u_resolution;
        return vec2(int(p.x * n_cols), int(p.y * n_rows));
    }

    float get_tile_index1d(float n_rows, float n_cols) {
        vec2 tile_index = get_tile_index2d(n_rows, n_cols);
        return tile_index.y * n_cols + tile_index.x;
    }


    vec2 rotate_space(vec2 p, float angle) {
        p -= 0.5;
        mat2 rotation = mat2(
            cos(angle), -sin(angle),
            sin(angle), cos(angle)
        );
        p = rotation * p + 0.5;
        return p;
    }

    float box(vec2 p) {
        if (min(p.x, p.y) >= 0.0 && max(p.x, p.y) <= 1.0) {
            return 1.0;
        } else {
            return 0.0;
        }
    }

    void main() {
        float grid_size = 5;
        float n_rows = grid_size;
        float n_cols = grid_size;
        float scale = 0.7071;
        vec2 p = gl_FragCoord.xy / u_resolution;
        float tile_idx = get_tile_index1d(n_rows, n_cols);
        float angle = sin(u_time * PI) * PI;
        if (int(tile_idx) % 2 == 0) {
            angle *= -1.0;
        }

        p = tile_space(p, n_rows, n_cols);
        p = scale_space(p, scale);
        p = rotate_space(p, angle);
        color = vec4(vec3(box(p)), 1.0);
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
