extern crate gfx_macros;

use gfx;
use render;

pub struct VertexBuffer {
    buffer: gfx::BufferHandle<Vertex>,
    batch: render::batch::RefBatch<_ShaderParamLink, ShaderParam>
}

#[shader_param(CubeBatch)]
pub struct ShaderParam {
    #[name = "projection"]
    pub projection: [[f32, ..4], ..4],
    #[name = "view"]
    pub view: [[f32, ..4], ..4],
    #[name = "s_texture"]
    pub s_texture: gfx::shade::TextureParam,
}

#[vertex_format]
pub struct Vertex {
    #[name = "position"]
    pub pos: [f32, ..3],
    #[name = "tex_coord"]
    pub tex_coord: [f32, ..2],
    #[name = "color"]
    pub color: [f32, ..3],
}

impl Clone for Vertex {
    fn clone(&self) -> Vertex {
        *self
    }
}

impl Vertex {
    pub fn new(pos: [f32, ..3], tex_coord: [f32, ..2], color: [f32, ..3]) -> Vertex {
        Vertex {
            pos: pos,
            tex_coord: tex_coord,
            color: color,
        }
    }
}

pub static VERTEX: gfx::ShaderSource = shaders! {
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

pub static FRAGMENT: gfx::ShaderSource = shaders!{
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
