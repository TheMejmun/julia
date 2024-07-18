use macroquad::prelude::*;

#[macroquad::main("Julia Set Vis by Saman Miran")]
async fn main() {
    let material = load_material(
        ShaderSource::Glsl {
            vertex: CRT_VERTEX_SHADER,
            fragment: CRT_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                ("resolution".to_string(), UniformType::Float2),
                ("mouse".to_string(), UniformType::Float2),
            ],
            ..MaterialParams::default()
        },
    )
    .unwrap();

    loop {
        clear_background(BLACK);

        material.set_uniform("resolution", vec2(screen_width(), screen_height()));
        let mouse = mouse_position();
        let x = (mouse.0 / screen_width()) * 2.0 - 1.0;
        let y = mouse.1 / screen_height() * 2.0 - 1.0;
        material.set_uniform("mouse", (x, y));

        gl_use_material(&material);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), BLACK);
        gl_use_default_material();

        next_frame().await;
        // println!("FPS: {}", get_fps());
    }
}

const CRT_FRAGMENT_SHADER: &'static str = r#"#version 330
precision highp float;

varying vec4 color;
varying vec2 uv;

uniform vec2 resolution;
uniform vec2 mouse;

const int iterations = 1024;
const float epsilon = 0.01;
const float scale = 3.0;

vec4 julia(vec2 pos) {
    float aspect_ratio = resolution.x / resolution.y;

    float c_r = mouse.x;
    float c_c = mouse.y;

    float z_r = pos.y * scale - scale / 2.0;
    float z_c = pos.x * aspect_ratio * scale - (scale * aspect_ratio) / 2.0;

    for (int i = 0; i < iterations; ++i) {
        float z_sq_r = z_r * z_r - z_c * z_c;
        float z_sq_c = z_r * z_c + z_r * z_c;

        float z_new_r = z_sq_r + c_r;
        float z_new_c = z_sq_c + c_c;

        float d = length(vec2(z_r - z_new_r, z_c - z_new_c));

        z_r = z_new_r;
        z_c = z_new_c;
        if (isinf(d) || isnan(d)) {
            float r = pow(1.0 / float(i), 0.2);
            float g = pow(1.0 / float(i), 0.4);
            float b = pow(1.0 / float(i), 0.8);
            return vec4(r, g, b, 1.0);
        }
    }

    return vec4(1.0, 1.0, 1.0, 1.0);
}

void main() {
    vec2 pos = uv;
    gl_FragColor = julia(pos);
}
"#;

const CRT_VERTEX_SHADER: &'static str = "#version 330
precision highp float;

attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";
