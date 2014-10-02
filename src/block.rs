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
