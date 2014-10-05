use std::collections::HashMap;
use std::cell::Cell;

use vertex::Vertex;
use block::{BlockType, Empty};
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

impl Block {
    pub fn new() -> Block {
        Block {
            block_type: Empty,
        }
    }
}

pub struct Chunk {
    pub blocks: [[[BlockType, ..16], ..16], ..16],
    pub buffers: Cell<Option<Buffer>>,
}

/// YZX
impl Chunk {
    pub fn new(cx: u16, cz: u16, cy: u16) -> Chunk {
        let mut blocks: Vec<Vertex> = Vec::with_capacity((CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE*36) as uint);
        for y in range(0, CHUNK_SIZE) {
            for z in range(0, CHUNK_SIZE) {
                for x in range(0, CHUNK_SIZE) {
                    blocks.push_all(
                        create_cube(
                            ((cx * CHUNK_SIZE) as f32) + x as f32,
                            ((cz * CHUNK_SIZE) as f32) + z as f32,
                            ((cy * CHUNK_SIZE) as f32) + y as f32,
                        )
                    );
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
    pub fn new(x: u16, z: u16) -> ChunkColumn {
        let mut chunks: Vec<Chunk> = Vec::new();
        for y in range(0u16, 5) {
            chunks.push(Chunk::new(x as u16, z as u16, y as u16))
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
    chunk_columns: HashMap<(u16, u16), ChunkColumn>
}

impl ChunkManager {
    pub fn new() -> ChunkManager {
        let mut chunk_columns = HashMap::new();
        for z in range(0, 3) {
            for x in range(0, 2) {
                chunk_columns.insert((x, z), ChunkColumn::new(x, z));
            }
        }
        ChunkManager {
            chunk_columns: chunk_columns,
        }
    }

    pub fn render(&self, buffer: &mut Vec<Vertex>) {
        for (_, chunk_column) in self.chunk_columns.iter() {
            chunk_column.render(buffer);
        }
    }
}
