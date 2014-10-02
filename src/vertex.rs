extern crate gfx_macros;

#[vertex_format]
pub struct Vertex {
    #[name = "a_Pos"]
    pub pos: [f32, ..3],

    #[name = "a_TexCoord"]
    pub tex_coord: [f32, ..2],

    #[name = "a_Color"]
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
