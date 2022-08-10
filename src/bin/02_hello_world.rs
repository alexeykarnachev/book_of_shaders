use book_of_shaders::Renderer;

const FRAGMENT_SHADER: &str = r#"
    #version 460

    out vec4 color;

    vec4 get_color() {
        return vec4(1.0, 1.0, 0.0, 1.0);
    }

    void main() {
        color = get_color();
    }
"#;

fn main() {
    let renderer = Renderer::from_fragment_shader(FRAGMENT_SHADER);
    renderer.draw();
}
