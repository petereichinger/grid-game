use bevy::prelude::*;

pub struct MeshData {
    pub positions: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
}

impl MeshData {
    pub fn create_triangle(&mut self, vertices: &[Vec3; 3]) {
        let index_offset: u32 = self
            .positions
            .len()
            .try_into()
            .expect("needs to fit into u32");

        self.positions.extend(vertices.iter());

        self.indices.push(index_offset);
        self.indices.push(index_offset + 1);
        self.indices.push(index_offset + 2);

        let a = (vertices[2] - vertices[0]).normalize_or(Vec3::Z);
        let b = (vertices[1] - vertices[0]).normalize_or(Vec3::Z);

        let normal: [f32; 3] = b.cross(a).into();

        self.normals.push(normal);
        self.normals.push(normal);
        self.normals.push(normal);

        self.uvs.push([0.0, 0.0].into());
        self.uvs.push([0.0, 0.0].into());
        self.uvs.push([0.0, 0.0].into());
        // TODO add uvs
    }
}
