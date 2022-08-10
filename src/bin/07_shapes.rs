use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460

    #define PIXELS_EPS 10.0
    #define PI 3.14159265359

    uniform vec2 u_resolution;
    uniform float u_time;

    out vec4 color;

    vec2 rotate_point(vec2 point, float angle, vec2 pivot) {
        vec2 p = point - pivot;
        float c = cos(angle);
        float s = sin(angle);

        p = vec2(p.x * c - p.y * s, p.y * c + p.x * s);

        p += pivot;

        return p;
    }

    float get_triangle_area(vec2 a, vec2 b, vec2 c) {
        return abs((b.x * a.y - a.x * b.y) + (c.x * b.y - b.x * c.y) + (a.x * c.y - c.x * a.y)) / 2.0;
    }

    float get_rectangle_area(vec2 a, vec2 b, vec2 c, vec2 d) {
        return get_triangle_area(a, b, c) + get_triangle_area(c, d, a);
    }

    bool is_in_rectangle(vec2 center, float vertical_d, float horizontal_d, float angle) {
        center *= u_resolution;
        float hr = (horizontal_d * u_resolution.x) / 2.0;
        float vr = (vertical_d * u_resolution.y) / 2.0;

        vec2 a = rotate_point(vec2(center.x - hr, center.y + vr), angle, center);
        vec2 b = rotate_point(vec2(center.x + hr, center.y + vr), angle, center);
        vec2 c = rotate_point(vec2(center.x + hr, center.y - vr), angle, center);
        vec2 d = rotate_point(vec2(center.x - hr, center.y - vr), angle, center);

        vec2 p = gl_FragCoord.xy;

        float rectangle_area = get_rectangle_area(a, b, c, d);
        float triangles_area = get_triangle_area(a, p, d) + get_triangle_area(d, p, c) + get_triangle_area(c, p, b) + get_triangle_area(p, b, a);
        if (abs(triangles_area - rectangle_area) < PIXELS_EPS) {
            return true;
        } else {
            return false;
        }
    }

    void main() {

        vec2 center = vec2(0.5, 0.5);

        float vertical_d = 0.5;
        float horizontal_d = (vertical_d * u_resolution.y) / u_resolution.x;

        float angle = u_time;

        if (is_in_rectangle(center, vertical_d, horizontal_d, angle)) {
            color = vec4(vec3(1.0), 1.0);
        } else {
            color = vec4(vec3(0.0), 1.0);
        }
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
