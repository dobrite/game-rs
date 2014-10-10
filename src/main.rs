#![feature(phase)]
#![crate_name = "game-rs"]

extern crate cgmath;
extern crate gfx;
extern crate piston;
extern crate glfw_game_window;
extern crate render;
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

pub mod chunk;
pub mod cube;
pub mod renderer;

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
    let program = device.link_program(renderer::VERTEX.clone(), renderer::FRAGMENT.clone()).unwrap();
    let state = gfx::DrawState::new().depth(gfx::state::LessEqual, true);

    let mut graphics = gfx::Graphics::new(device);

    let mut params = renderer::ShaderParam {
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

    let mut chunk_manager: chunk::ChunkManager = chunk::ChunkManager::new();
    let mut staging_buffer = vec![];
    let mut pending_buffer = vec![];

    for cy in range(0u8, 16) {
        for cz in range(0u8, 16) {
            for cx in range(0u8, 16) {
                // chunk coords or block coords?
                chunk_manager.create_chunk(cx as i32, cz as i32, cy as i32);
            }
        }
    }

    chunk_manager.each_chunk(|cx, cy, cz, chunk, buffer| {
        pending_chunks.push((cx, cy, cz, chunk, buffer))
    });

    for e in game_iter {
        match e {
            piston::Render(args) => {
                chunk_manager.fill_buffer(0i32, 0, 0, &mut staging_buffer);
                let data = staging_buffer.as_slice();
                let buf = graphics.device.create_buffer(data.len(), gfx::UsageStatic);
                graphics.device.update_buffer(buf, data, 0);
                let mesh = gfx::Mesh::from_format(buf, data.len() as u32);
                graphics.clear(clear_data, gfx::Color | gfx::Depth, &frame);
                let batch: renderer::CubeBatch = graphics.make_batch(
                    &program,
                    &mesh,
                    mesh.to_slice(gfx::TriangleList),
                    &state
                ).unwrap();
                params.view = first_person.camera(0.0).orthogonal();
                graphics.draw(&batch, &params, &frame);
                graphics.end_frame();
            },
            piston::Update(args) => {
                pending = pending_chunks.pop();
                match pending {
                    Some((cx, cy, cz, chunk, buffer)) => {
                        //match buffer.get {
                        //    Some(buffer) => //delete buffer
                        //    None ={}
                        //}
                    }
                    None => {}
                }
                first_person.update(args.dt);
            },
            piston::Input(e) => first_person.input(&e),
        }
    }
}
