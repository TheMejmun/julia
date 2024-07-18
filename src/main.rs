use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

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
                ("c".to_string(), UniformType::Float2),
                ("iterations".to_string(), UniformType::Int1),
                ("scale".to_string(), UniformType::Float1),
            ],
            ..MaterialParams::default()
        },
    )
    .unwrap();

    let mut c_locked = false;
    let mut c_r: f32 = 0.0;
    let mut c_c: f32 = 0.0;
    let mut scale: f32 = 3.0;
    let mut iterations: i32 = 512;
    let mut iterations_input = iterations.to_string();

    loop {
        clear_background(BLACK);

        if iterations_input.trim().len() > 0 {
            iterations = match iterations_input.trim().parse::<i32>() {
                Ok(num) => clamp(num, 0, 16384),
                Err(_) => iterations,
            };
            iterations_input = iterations.to_string();
        }
        if is_key_pressed(KeyCode::Space) {
            c_locked = !c_locked;
        }

        if !c_locked {
            let mouse = mouse_position();
            c_r = mouse.0 / screen_width() * 2.0 - 1.0;
            c_c = mouse.1 / screen_height() * 2.0 - 1.0;
        }
        material.set_uniform("c", (c_r, c_c));
        material.set_uniform("resolution", vec2(screen_width(), screen_height()));
        material.set_uniform("iterations", iterations);
        material.set_uniform("scale", scale);

        gl_use_material(&material);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), BLACK);
        gl_use_default_material();

        widgets::Window::new(hash!(), vec2(10., 10.), vec2(200., 120.))
            .label("Settings")
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.input_text(hash!(), "Iterations", &mut iterations_input);
                ui.slider(
                    hash!(),
                    "scale",
                    std::ops::Range::<f32> {
                        start: 0.01,
                        end: 4.0,
                    },
                    &mut scale,
                );
                ui.label(
                    Vec2::new(0., 50.),
                    format!("c: ({:.2$} + {:.2$}i)", c_r, c_c, 4).as_str(),
                );
                ui.label(Vec2::new(0., 70.), "Press space to lock/unlock .");
                ui.label(Vec2::new(0., 90.), format!("FPS: {}", get_fps()).as_str());
            });

        next_frame().await;
        // println!("FPS: {}", get_fps());
    }
}

const CRT_FRAGMENT_SHADER: &'static str = r#"#version 330
precision highp float;

varying vec2 uv;

uniform vec2 resolution;
uniform vec2 c;
uniform int iterations;
uniform float scale;

const float epsilon = 0.0001;

vec4 julia(vec2 pos) {
    float aspect_ratio = resolution.x / resolution.y;

    float c_r = c.x;
    float c_c = c.y;

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
        } else if  (d < epsilon) {
            return vec4(1.0, 1.0, 1.0, 1.0);
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

varying lowp vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";
