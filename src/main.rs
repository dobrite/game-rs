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
    #[name = "projection"]
    pub projection: [[f32, ..4], ..4],
    #[name = "view"]
    pub view: [[f32, ..4], ..4],
    #[name = "s_texture"]
    pub s_texture: gfx::shade::TextureParam,
}

static VERTEX: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120
    uniform mat4 projection, view;

    attribute vec2 tex_coord;
    attribute vec3 color, position;

    varying vec2 v_tex_coord;
    varying vec3 v_color;

    void main() {
        v_tex_coord = tex_coord;
        v_color = color;
        gl_Position = projection * view * vec4(position, 1.0);
    }
"
GLSL_150: b"
    #version 150 core
    uniform mat4 projection, view;

    in vec2 tex_coord;
    in vec3 color, position;

    out vec2 v_tex_coord;
    out vec3 v_color;

    void main() {
        v_tex_coord = tex_coord;
        v_color = color;
        gl_Position = projection * view * vec4(position, 1.0);
    }
"
};

static FRAGMENT: gfx::ShaderSource = shaders!{
GLSL_120: b"
    #version 120

    uniform sampler2D s_texture;

    varying vec2 v_tex_coord;
    varying vec3 v_color;

    void main() {
        vec4 tex_color = texture2D(s_texture, v_tex_coord);
        float blend = dot(v_tex_coord-vec2(0.5,0.5), v_tex_coord-vec2(0.5,0.5));
        gl_FragColor = mix(tex_color, vec4(0.0,0.0,0.0,0.0), blend*1.0);
    }
"
GLSL_150: b"
    #version 150 core
    out vec4 out_color;

    uniform sampler2D s_texture;

    in vec2 v_tex_coord;
    in vec3 v_color;

    void main() {
        vec4 tex_color = texture(s_texture, v_tex_coord);
        float blend = dot(v_tex_coord-vec2(0.5,0.5), v_tex_coord-vec2(0.5,0.5));
        out_color = mix(tex_color, vec4(0.0,0.0,0.0,0.0), blend*1.0);
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

    let mut chunks = Vec::new();
    let chunk_manager: chunk::ChunkManager = chunk::ChunkManager::new();
    for (_, chunk_column) in chunk_manager.chunk_columns.iter() {
        chunk_column.render(chunks);
    }

    let data = chunks.as_slice();

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
    let program = device.link_program(VERTEX.clone(), FRAGMENT.clone()).unwrap();
    let state = gfx::DrawState::new().depth(gfx::state::LessEqual, true);

    let mut graphics = gfx::Graphics::new(device);

    let batch: CubeBatch = graphics.make_batch(&program, &mesh, mesh.to_slice(gfx::TriangleList), &state).unwrap();

    let mut params = Params {
        projection: [[0.0, ..4], ..4],
        view: [[0.0, ..4], ..4],
        s_texture: (texture, Some(sampler)),
    };

    let clear_data = gfx::ClearData {
        color: [0.3, 0.3, 0.3, 1.0],
        depth: 1.0,
        stencil: 0,
    };

    let mut first_person_settings = cam::FirstPersonSettings::keyboard_wasd();
    first_person_settings.speed_horizontal = 100.0;
    first_person_settings.speed_vertical = 100.0;
    let mut first_person = cam::FirstPerson::new(
        [200.0f32, 200.0, 200.0],
        first_person_settings
    );

    params.projection = cam::CameraPerspective {
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
                graphics.clear(clear_data, gfx::Color | gfx::Depth, &frame);
                params.view = first_person.camera(0.0).orthogonal();
                graphics.draw(&batch, &params, &frame);
                graphics.end_frame();
            },
            piston::Update(args) => first_person.update(args.dt),
            piston::Input(e) => first_person.input(&e),
        }
    }
}
