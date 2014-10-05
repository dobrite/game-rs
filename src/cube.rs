use renderer::Vertex;

pub fn create_cube(x: f32, z: f32, y: f32, buffer: &mut Vec<Vertex>) {
    //top (0, 0, 1)
    buffer.push(Vertex::new([0.0f32 + x, 0.0f32 + y, 1.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 0.0f32 + y, 1.0f32 + z], [1f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 1.0f32 + y, 1.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 1.0f32 + y, 1.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 1.0f32 + y, 1.0f32 + z], [0f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 0.0f32 + y, 1.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
    //bottom (0, 0, 0)
    buffer.push(Vertex::new([1.0f32 + x, 1.0f32 + y, 0.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 1.0f32 + y, 0.0f32 + z], [1f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 0.0f32 + y, 0.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 0.0f32 + y, 0.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 0.0f32 + y, 0.0f32 + z], [0f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 1.0f32 + y, 0.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
    //right (1, 0, 0)
    buffer.push(Vertex::new([1.0f32 + x, 0.0f32 + y, 0.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 1.0f32 + y, 0.0f32 + z], [1f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 1.0f32 + y, 1.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 1.0f32 + y, 1.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 0.0f32 + y, 1.0f32 + z], [0f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 0.0f32 + y, 0.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
    //left (0.0, 0, 0)
    buffer.push(Vertex::new([0.0f32 + x, 1.0f32 + y, 1.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 0.0f32 + y, 1.0f32 + z], [1f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 0.0f32 + y, 0.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 0.0f32 + y, 0.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 1.0f32 + y, 0.0f32 + z], [0f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 1.0f32 + y, 1.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
    //front (0, 1, 0)
    buffer.push(Vertex::new([0.0f32 + x, 1.0f32 + y, 0.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 1.0f32 + y, 0.0f32 + z], [1f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 1.0f32 + y, 1.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 1.0f32 + y, 1.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 1.0f32 + y, 1.0f32 + z], [0f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 1.0f32 + y, 0.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
    //back (0, 0.0, 0)
    buffer.push(Vertex::new([1.0f32 + x, 0.0f32 + y, 1.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 0.0f32 + y, 1.0f32 + z], [1f32, 0f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 0.0f32 + y, 0.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([0.0f32 + x, 0.0f32 + y, 0.0f32 + z], [1f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 0.0f32 + y, 0.0f32 + z], [0f32, 1f32], [255f32, 255f32, 255f32]));
    buffer.push(Vertex::new([1.0f32 + x, 0.0f32 + y, 1.0f32 + z], [0f32, 0f32], [255f32, 255f32, 255f32]));
}
