use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460
    // HSL and HSV: https://en.wikipedia.org/wiki/HSL_and_HSV

    uniform float u_time;
    uniform vec2 u_resolution;
    uniform vec2 u_cursor;

    float PI = 3.14159265359;

    out vec4 color;

    vec3 rgb2hsb(in vec3 c){
        vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
        vec4 p = mix(
            vec4(c.bg, K.wz),
            vec4(c.gb, K.xy),
            step(c.b, c.g)
        );
        vec4 q = mix(
            vec4(p.xyw, c.r),
            vec4(c.r, p.yzx),
            step(p.x, c.r)
        );
        float d = q.x - min(q.w, q.y);
        float e = 1.0e-10;
        return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
    }

    // Function from Iñigo Quiles
    // https://www.shadertoy.com/view/MsS3Wc
    vec3 hsb2rgb(in vec3 c){
        vec3 rgb = clamp(abs(mod(c.x * 6.0 + vec3(0.0, 4.0, 2.0), 6.0) - 3.0 ) - 1.0, 0.0, 1.0);
        rgb = rgb * rgb * (3.0 - 2.0 * rgb);
        return c.z * mix(vec3(1.0), rgb, c.y);
    }


    void main() {
        vec2 coords = gl_FragCoord.xy / u_resolution;

        vec2 center = vec2(u_cursor.x, u_cursor.y) / u_resolution;
        vec2 to_center = center - coords;
        float angle = (atan(to_center.y, to_center.x) + u_time);
        angle = angle - float(int((angle + u_time) / 0.5 * PI));
        float radius = length(to_center) * 2.0;
        
        vec3 hsb = vec3((angle / 2 * PI) + 0.5, radius, 1.0);
        vec3 rgb = hsb2rgb(hsb);
        color = vec4(rgb, 1.0);
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
