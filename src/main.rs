#![feature(phase)]
#![crate_name = "game-rs"]

extern crate cgmath;
extern crate gfx;
extern crate piston;
extern crate glfw_game_window;
#[phase(plugin)]
extern crate gfx_macros;
extern crate glfw;
extern crate native;
extern crate time;

use std::f32::consts::PI;
use cgmath::FixedArray;
use cgmath::{Matrix, Point3, Vector3};
use cgmath::{Transform, AffineMatrix3};
use gfx::{Device, DeviceHelper, ToSlice};
use glfw::Context;
use glfw_game_window::WindowGLFW;
use piston::{cam, Window};

pub mod block;
pub mod chunk;
pub mod vertex;
pub mod cube;

// The shader_param attribute makes sure the following struct can be used to
// pass parameters to a shader. Its argument is the name of the type that will
// be generated to represent your the program. Search for `CubeBatch` below, to
// see how it's used.
#[shader_param(CubeBatch)]
struct Params {
    #[name = "u_Transform"]
    transform: [[f32, ..4], ..4],

    #[name = "t_Color"]
    color: gfx::shade::TextureParam,
}

static VERTEX_SRC: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120

    attribute vec3 a_Pos;
    attribute vec2 a_TexCoord;
    varying vec2 v_TexCoord;

    uniform mat4 u_Transform;

    void main() {
        v_TexCoord = a_TexCoord;
        gl_Position = u_Transform * vec4(a_Pos, 1.0);
    }
"
GLSL_150: b"
    #version 150 core

    in vec3 a_Pos;
    in vec2 a_TexCoord;
    out vec2 v_TexCoord;

    uniform mat4 u_Transform;

    void main() {
        v_TexCoord = a_TexCoord;
        gl_Position = u_Transform * vec4(a_Pos, 1.0);
    }
"
};

static FRAGMENT_SRC: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120

    varying vec2 v_TexCoord;
    uniform sampler2D t_Color;

    void main() {
        vec4 tex = texture2D(t_Color, v_TexCoord);
        float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
        gl_FragColor = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
    }
"
GLSL_150: b"
    #version 150 core

    in vec2 v_TexCoord;
    out vec4 o_Color;

    uniform sampler2D t_Color;
    void main() {
        vec4 tex = texture(t_Color, v_TexCoord);
        float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
        o_Color = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
    }
"
};

//----------------------------------------

// We need to run on the main thread, so ensure we are using the `native` runtime. This is
// technically not needed, since this is the default, but it's not guaranteed.
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
     native::start(argc, argv, main)
}

fn main() {

    let (win_width, win_height) = (2560, 1600);
    let mut window_glfw = WindowGLFW::new(
        piston::shader_version::opengl::OpenGL_3_2,
        piston::WindowSettings {
            title: "game-rs".to_string(),
            size: [win_width, win_height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );

    window_glfw.capture_cursor(true);

    let (w, h) = window_glfw.get_size();
    let frame = gfx::Frame::new(w as u16, h as u16);

    let mut device = gfx::GlDevice::new(|s| window_glfw.window.get_proc_address(s));
    //let chunk: chunk::Chunk = chunk::Chunk::new();
    //let data = chunk.render();

    let mut chunks = Vec::new();
    let chunk_column: chunk::ChunkColumn = chunk::ChunkColumn::new();
    chunk_column.render(&mut chunks);

    let data = chunks.as_slice();
    //let data = chunk.blocks.as_slice();

    let buf = device.create_buffer(data.len(), gfx::UsageStatic);
    device.update_buffer(buf, data, 0);
    let mesh = gfx::Mesh::from_format(buf, data.len() as u32);

    let texture_info = gfx::tex::TextureInfo {
        width: 1,
        height: 1,
        depth: 1,
        levels: 1,
        kind: gfx::tex::Texture2D,
        format: gfx::tex::RGBA8,
    };

    let image_info = texture_info.to_image_info();
    let texture = device.create_texture(texture_info).unwrap();
    device.update_texture(&texture, &image_info, [0x20u8, 0xA0u8, 0xC0u8, 0x00u8]).unwrap();

    let sampler = device.create_sampler(gfx::tex::SamplerInfo::new(gfx::tex::Bilinear, gfx::tex::Clamp));
    let program = device.link_program(VERTEX_SRC.clone(), FRAGMENT_SRC.clone()).unwrap();
    let state = gfx::DrawState::new().depth(gfx::state::LessEqual, true);

    let mut graphics = gfx::Graphics::new(device);

    let batch: CubeBatch = graphics.make_batch(&program, &mesh, mesh.to_slice(gfx::TriangleList), &state).unwrap();

    let mut data = Params {
        transform: piston::vecmath::mat4_id(),
        color: (texture, Some(sampler)),
    };

    let clear_data = gfx::ClearData {
        color: [0.3, 0.3, 0.3, 1.0],
        depth: 1.0,
        stencil: 0,
    };

    let model = piston::vecmath::mat4_id();

    let mut first_person_settings = cam::FirstPersonSettings::keyboard_wasd();
    first_person_settings.speed_horizontal = 8.0;
    first_person_settings.speed_vertical = 4.0;
    let mut first_person = cam::FirstPerson::new(
        [200.0f32, 200.0, 200.0],
        first_person_settings
    );
    first_person.direction = [0.0f32, 0.0, 0.0];

    let projection = cam::CameraPerspective {
        fov: 70.0f32,
        near_clip: 0.1,
        far_clip: 1000.0,
        aspect_ratio: {
            let (w, h) = window_glfw.get_size();
            (w as f32) / (h as f32)
        }
    }.projection();

    let mut game_iter = piston::EventIterator::new(
        &mut window_glfw,
        &piston::EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        }
    );

    for e in game_iter {
        match e {
            piston::Render(args) => {
                graphics.clear(
                    clear_data,
                    gfx::Color | gfx::Depth,
                    &frame
                );
                data.transform = cam::model_view_projection(
                    model,
                    first_person.camera(args.ext_dt).orthogonal(),
                    projection,
                );
                graphics.draw(&batch, &data, &frame);
                graphics.end_frame();
            },
            piston::Update(args) => first_person.update(args.dt),
            piston::Input(e) => first_person.input(&e),
        }
    }
}
