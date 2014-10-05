use std::collections::HashMap;
use std::cell::Cell;
use renderer::VertexBuffer;

use renderer::Vertex;
use cube::create_cube;

static CHUNK_SIZE: u16 = 16;
static WORLD_HEIGHT: u16 = 256;

pub enum BlockType {
    Empty,
    Grass,
    Dirt,
}

pub struct Block {
    block_type: BlockType,
}

pub struct Chunk {
    pub blocks: [[[Block, ..16], ..16], ..16],
    pub buffer: Cell<Option<VertexBuffer>>,
}

impl Clone for Chunk {
    fn clone(&self) -> Chunk {
        *self
    }
}

pub struct ChunkColumn {
    pub chunks: Vec<Chunk>
}

pub struct ChunkManager {
    chunk_columns: HashMap<(i32, i32), ChunkColumn>
}

//pub struct Buffer {
//    buffer: gfx::BufferHandle<Vertex>,
//    batch: render::batch::RefBatch<_ShaderParamLink, ShaderParam>
//}

impl ChunkManager {
    pub fn new() -> ChunkManager {
        ChunkManager {
            chunk_columns: HashMap::new(),
        }
    }

    pub fn add_chunk_column(&mut self, cx: i32, cz: i32, c: ChunkColumn) {
        self.chunk_columns.insert((cx, cz), c);
    }

    pub fn create_chunk_column(&mut self, cx: i32, cz: i32) {
        let mut chunks = Vec::new();
        for i in range(0u8, 16) {
            chunks.push(Chunk {
                blocks: [[[Block { block_type: Dirt }, ..16], ..16], ..16],
                buffer: Cell::new(None),
            });
        }
        let c = ChunkColumn {
            chunks: chunks,
        };
        self.chunk_columns.insert((cx, cz), c);
    }

    pub fn fill_buffer(&self, cx: i32, cz: i32, cy: i32, vbuffer: &mut Vec<Vertex>) {
        for y in range(0u8, 16) {
            for z in range(0u8, 16) {
                for x in range(0u8, 16) {
                    create_cube(
                        (cx as f32 * 16f32) + x as f32,
                        (cz as f32 * 16f32) + z as f32,
                        (cy as f32 * 16f32) + y as f32,
                        vbuffer,
                    );
                }
            }
        }
    }
}
