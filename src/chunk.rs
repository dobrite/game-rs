use std::collections::HashMap;

use vertex::Vertex;
use block::{BlockType, Empty};
use cube::create_cube;

static CHUNK_SIZE: u8 = 16;

pub struct Chunk {
    pub blocks: Vec<Vertex>,
}

/// YZX
impl Chunk {
    pub fn new() -> Chunk {
        let mut blocks: Vec<Vertex> = Vec::with_capacity((CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE*36) as uint);
        for y in range(0, CHUNK_SIZE) {
            for z in range(0, CHUNK_SIZE) {
                for x in range(0, CHUNK_SIZE) {
                    blocks.push_all(create_cube(x as f32, y as f32, z as f32));
                }
            }
        }

        Chunk {
            blocks: blocks,
        }
    }
}

pub struct ChunkColumn {
    pub chunks: Vec<Chunk>
}

pub struct ChunkManager {
    chunk_columns: HashMap<(i32, i32), ChunkColumn>
}

impl ChunkManager {
    pub fn new() -> ChunkManager {
        ChunkManager {
            chunk_columns: HashMap::new()
        }
    }
}
