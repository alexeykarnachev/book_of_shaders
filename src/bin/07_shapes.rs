use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460

    #define PIXELS_EPS 10.0
    #define PI 3.14159265359

    uniform vec2 u_resolution;
    uniform vec2 u_cursor;
    uniform float u_time;

    out vec4 color;

    struct Rectangle {
        vec2 center;
        float v_diam;
        float h_diam_ratio;
        float angle_speed;
    };

    Rectangle rectangles[4] = Rectangle[4](
        Rectangle(vec2(0.3, 0.3), 0.4, 1.0, 1.0),
        Rectangle(vec2(0.7, 0.3), 0.4, 1.0, -1.0),
        Rectangle(vec2(0.3, 0.7), 0.4, 1.0, -1.0),
        Rectangle(vec2(0.7, 0.7), 0.4, 1.0, 1.0)
    );

    float get_triangle_area(vec2 a, vec2 b, vec2 c) {
        return abs((b.x * a.y - a.x * b.y) + (c.x * b.y - b.x * c.y) + (a.x * c.y - c.x * a.y)) / 2.0;
    }

    float get_rectangle_area(vec2 a, vec2 b, vec2 c, vec2 d) {
        return get_triangle_area(a, b, c) + get_triangle_area(c, d, a);
    }

    vec2 rotate_point(vec2 point, float angle, vec2 pivot) {
        vec2 p = point - pivot;
        float c = cos(angle);
        float s = sin(angle);

        p = vec2(p.x * c - p.y * s, p.y * c + p.x * s);

        p += pivot;

        return p;
    }

    bool is_in_rectangle(Rectangle rectangle) {
        vec2 p = gl_FragCoord.xy;

        vec2 center = rectangle.center * u_resolution;
        float angle = u_time * rectangle.angle_speed;
        float vd = rectangle.v_diam;
        float hd = (vd * u_resolution.y * rectangle.h_diam_ratio) / u_resolution.x;
        float hr = (hd * u_resolution.x) / 2.0;
        float vr = (vd * u_resolution.y) / 2.0;

        vec2 a = rotate_point(vec2(center.x - hr, center.y + vr), angle, center);
        vec2 b = rotate_point(vec2(center.x + hr, center.y + vr), angle, center);
        vec2 c = rotate_point(vec2(center.x + hr, center.y - vr), angle, center);
        vec2 d = rotate_point(vec2(center.x - hr, center.y - vr), angle, center);

        // Check if point in the rectangle:
        float rectangle_area = get_rectangle_area(a, b, c, d);
        float triangles_area = get_triangle_area(a, p, d) + get_triangle_area(d, p, c) + get_triangle_area(c, p, b) + get_triangle_area(p, b, a);
        if (abs(triangles_area - rectangle_area) < PIXELS_EPS) {
            return true;
        } else {
            return false;
        }
    }

    void main() {
        color = vec4(vec3(0.0), 1.0);
        int n_rects_in_point = 0;
        for (int i = 0; i < rectangles.length(); ++i) {
            if (is_in_rectangle(rectangles[i])) {
                n_rects_in_point += 1;
            }
        }

        Rectangle cursor_rectangle = Rectangle(u_cursor / u_resolution, 0.1, 1.0, 5.0);
        if (is_in_rectangle(cursor_rectangle)) {
            n_rects_in_point += 1;
        }

        if (n_rects_in_point == 1) {
            color = vec4(vec3(1.0), 1.0);
        } else if (n_rects_in_point > 1) {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
