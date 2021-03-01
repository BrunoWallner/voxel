use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub struct Triangle {}
impl Triangle {
    pub fn new(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) -> Vec<Vertex> {
        let vertex1 = Vertex { position: [p1[0], p1[1]] };
        let vertex2 = Vertex { position: [p2[0], p2[1]] };
        let vertex3 = Vertex { position: [p3[0], p3[1]] };
        vec![vertex1, vertex2, vertex3]
    }
}