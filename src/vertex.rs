extern crate gfx_macros;

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
