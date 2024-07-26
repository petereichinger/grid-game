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

        self.normals.extend(std::iter::repeat(normal).take(3));

        self.uvs.extend(std::iter::repeat([0.0, 0.0]).take(3));
        // TODO: add uvs
    }

    pub fn create_quad(&mut self, vertices: &[Vec3; 4]) {
        let index_offset: u32 = self
            .positions
            .len()
            .try_into()
            .expect("must be a valid u32");

        self.positions.extend(vertices);

        let (o1, o2, o3, o4, o5, o6) = (0, 2, 1, 1, 2, 3);

        self.indices.push(index_offset + o1);
        self.indices.push(index_offset + o2);
        self.indices.push(index_offset + o3);

        self.indices.push(index_offset + o4);
        self.indices.push(index_offset + o5);
        self.indices.push(index_offset + o6);

        let a = (vertices[2] - vertices[0]).normalize_or(Vec3::Z);
        let b = (vertices[1] - vertices[0]).normalize_or(Vec3::Z);

        let normal: [f32; 3] = a.cross(b).into();

        self.normals.extend(std::iter::repeat(normal).take(4));
        // TODO:: UVs
        self.uvs.extend(std::iter::repeat([0.0, 0.0]).take(4));
    }
}
