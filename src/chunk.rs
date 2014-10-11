use std::collections::HashMap;
use std::cell::Cell;

use renderer::{Vertex, VertexBuffer};
use cube::create_cube;

pub static CHUNK_SIZE: uint = 8;
pub static WORLD_HEIGHT: u16 = 256;

pub enum BlockType {
    Empty,
    Grass,
    Dirt,
}

pub struct Block {
    block_type: BlockType,
}

pub struct Chunk {
    pub blocks: [[[Block, ..CHUNK_SIZE], ..CHUNK_SIZE], ..CHUNK_SIZE],
    pub buffer: Cell<Option<VertexBuffer>>,
    //fn create_mesh // iterate through blocks calling create_cube
    //fn create_cube //create vertexes
    //fn render translate position and call render_mesh
}

impl Clone for Chunk {
    fn clone(&self) -> Chunk {
        *self
    }
}

impl Chunk {
    pub fn fill_buffer(&self, cx: i32, cz: i32, cy: i32, vbuffer: &mut Vec<Vertex>) {
        for y in range(0u, CHUNK_SIZE) {
            for z in range(0u, CHUNK_SIZE) {
                for x in range(0u, CHUNK_SIZE) {
                    create_cube(
                        (cx as f32 * CHUNK_SIZE as f32) + x as f32,
                        (cz as f32 * CHUNK_SIZE as f32) + z as f32,
                        (cy as f32 * CHUNK_SIZE as f32) + y as f32,
                        vbuffer,
                    );
                }
            }
        }
    }
}

pub struct ChunkColumn {
    pub chunks: Vec<Chunk>
}

pub struct ChunkManager {
    chunks: HashMap<(i32, i32, i32), Chunk>,
}

//pub struct Buffer {
//    buffer: gfx::BufferHandle<Vertex>,
//    batch: render::batch::RefBatch<_ShaderParamLink, ShaderParam>
//}

impl ChunkManager {
    pub fn new() -> ChunkManager {
        ChunkManager {
            chunks: HashMap::new(),
        }
    }

    /*
     * pub fun update(f32 dt, vec3 camera_position, vec3 camera_view) {
     *   update_async_chunker() ?
     *   update_load_list()
     *   update_setup_list()
     *   update_rebuild_list()
     *   update_flags_list
     *   update_unload_list
     *   update_visibility_list(camera_position)
     *   if(last_camera_position != camera_position || last_camera_view != camera_view) {
     *     update_render_list()
     *   }
     *
     *   last_camera_position = camera_position
     *   last_camera_view = camera_view
     * }
    */

    /*
     * pub fn update_load_list() {
     *   num_chunks_loaded = 0
     *   iterate over all chunks calling load if not loaded
     *   break early when num_chunks_loaded limit is reached
     *   clear update_load_list each frame (reupdated in update_visible_list)
     * }
    */

    /*
     * pub fn update_setup_list() {
     *   iterate over setup_list calling setup on any chunk loaded and not setup
     *   clear list each frame (reupdated in update_visiblity_list)
     * }
    */

    pub fn create_chunk(&mut self, cx: i32, cz: i32, cy: i32) {
        self.chunks.insert((cx, cz, cy), Chunk {
            blocks: [[[Block { block_type: Dirt }, ..CHUNK_SIZE as uint], ..CHUNK_SIZE as uint], ..CHUNK_SIZE as uint],
            buffer: Cell::new(None),
        });
    }

    pub fn each_chunk<'a>(&'a self, f: |cx: i32, cy: i32, cz: i32, c: &'a Chunk, b: Option<VertexBuffer>|) {
        for (&(cx, cz, cy), chunk) in self.chunks.iter() {
            f(cx, cy, cz, chunk, chunk.buffer.get())
        }
    }
}
