use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460
    #define PI 3.14159265359

    uniform float u_time;
    uniform vec2 u_resolution;
    uniform vec2 u_cursor;

    out vec4 color;

    float random(vec2 p) {
        return fract(
            sin(
                dot(p, vec2(12.9898, 78.233))
            ) * (43758.543123 + floor(u_time))
        );
    }

    float truchet(vec2 p, float seed) {
        if (seed < 0.5) {
            return step(abs(p.x - p.y), 0.05);
        } else {
            return step(abs(1.0 - (p.x + p.y)), 0.05);
        }
    }

    void main() {
        vec2 p = gl_FragCoord.xy / u_resolution;

        p *= 30;
        vec2 ipos = floor(p);
        vec2 fpos = fract(p);
        float seed = random(ipos);

        color = vec4(
            vec3(truchet(fpos, seed)),
            1.0
        );
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
