use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460
    #define PI 3.14159265359

    uniform float u_time;
    uniform vec2 u_resolution;
    uniform vec2 u_cursor;

    out vec4 color;

    vec2 rotate_point2d(vec2 p, float angle) {
        p -= 0.5;
        mat2 rotation = mat2(
            cos(angle), -sin(angle),
            sin(angle), cos(angle)
        );
        p = rotation * p + 0.5;
        return p;
    }

    vec2 translate_point2d(vec2 p, vec2 t) {
        // Since we are translating the space in this shader, we need to take translation
        // with the opposite sign in order to translate the point.
        mat3 translation = mat3(
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            -t.x, -t.y, 1.0
        );
        p = (translation * vec3(p, 1.0)).xy;
        return p;
    }

    vec2 scale_point2d(vec2 p, vec2 s) {
        // Since we are scaling the space in this shader, we need to inverse scale coefficients
        // in order to scale the point.
        p -= 0.5;
        mat2 scale = mat2(
            1.0 / s.x, 0.0,
            0.0, 1.0 / s.y
        );
        p = scale * p;
        p += 0.5;
        return p;
    }

    float rectangle(vec2 p, float hd, float vd) {
        vec2 r = vec2(hd, vd) / 2.0;
        vec2 c = vec2(0.5);

        float s = 0.01;
        vec2 col = smoothstep(c - r, c - r + s, p) * smoothstep(1 - c - r, 1 - c - r + s, 1 - p);
        return col.x * col.y;
    }

    float cross(vec2 p, float hd, float vd) {
        float h_bar = rectangle(p, hd, vd);
        float v_bar = rectangle(p, vd, hd);
        return h_bar + v_bar;
    }

    void main() {
        vec2 p = gl_FragCoord.xy / u_resolution;
        p = scale_point2d(p, vec2(sin(u_time * 2.0), cos(u_time * 2.0)));
        p = translate_point2d(p, vec2(sin(u_time) / 4.0, cos(u_time) / 4.0));
        p = rotate_point2d(p, sin(u_time * 2.0) * PI);
        color = vec4(vec3(cross(p, 0.1, 0.03)), 1.0);
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
