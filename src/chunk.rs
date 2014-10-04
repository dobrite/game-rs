use std::collections::HashMap;

use vertex::Vertex;
use block::{BlockType, Empty};
use cube::create_cube;

static CHUNK_SIZE: u16 = 16;
static WORLD_HEIGHT: u16 = 256;

pub struct Chunk {
    pub blocks: Vec<Vertex>,
}

/// YZX
impl Chunk {
    pub fn new(cy: u16) -> Chunk {
        let mut blocks: Vec<Vertex> = Vec::with_capacity((CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE*36) as uint);
        for y in range(0, CHUNK_SIZE) {
            for z in range(0, CHUNK_SIZE) {
                for x in range(0, CHUNK_SIZE) {
                    blocks.push_all(create_cube(x as f32, y as f32, ((cy * CHUNK_SIZE) as f32) + z as f32));
                }
            }
        }

        Chunk {
            blocks: blocks,
        }
    }

    pub fn render(&self) -> &[Vertex] {
        self.blocks.as_slice()
    }
}

pub struct ChunkColumn {
    pub chunks: Vec<Chunk>
}

impl ChunkColumn {
    pub fn new() -> ChunkColumn {
        let mut chunks: Vec<Chunk> = Vec::with_capacity((WORLD_HEIGHT / CHUNK_SIZE) as uint);
        for y in range(0, WORLD_HEIGHT / CHUNK_SIZE) {
            chunks.push(Chunk::new(y as u16))
        };
        ChunkColumn {
            chunks: chunks
        }
    }

    pub fn render(&self, buffer: &mut Vec<Vertex>) {
        for chunk in self.chunks.iter() {
            buffer.push_all(chunk.render());
        }
    }
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

    pub fn add_chunk_column(&mut self, x: i32, z: i32, c: ChunkColumn) {
        self.chunk_columns.insert((x, z), c);
    }
}
