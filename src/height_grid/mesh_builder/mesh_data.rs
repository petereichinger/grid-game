use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

#[derive(Clone, Default)]
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

        let a = (vertices[1] - vertices[0]).normalize_or(Vec3::X);
        let b = (vertices[2] - vertices[0]).normalize_or(Vec3::Y);

        let normal: [f32; 3] = a.cross(b).into();

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

        self.indices.push(index_offset);
        self.indices.push(index_offset + 2);
        self.indices.push(index_offset + 1);

        self.indices.push(index_offset + 1);
        self.indices.push(index_offset + 2);
        self.indices.push(index_offset + 3);

        let a = (vertices[2] - vertices[0]).normalize_or(Vec3::X);
        let b = (vertices[1] - vertices[0]).normalize_or(Vec3::Y);

        let normal: [f32; 3] = a.cross(b).into();

        self.normals.extend(std::iter::repeat(normal).take(4));
        // TODO:: UVs
        self.uvs.extend(std::iter::repeat([0.0, 0.0]).take(4));
    }
}

impl From<MeshData> for Mesh {
    fn from(value: MeshData) -> Self {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_indices(Indices::U32(value.indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, value.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, value.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, value.uvs)
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec3;

    use super::MeshData;

    #[test]
    fn create_triangle_works() {
        let mut mesh_data = MeshData::default();

        mesh_data.create_triangle(&[Vec3::ZERO, Vec3::X, Vec3::Y]);

        assert_eq!(mesh_data.positions.len(), 3);
        assert_eq!(mesh_data.indices.len(), 3);
        assert_eq!(mesh_data.uvs.len(), 3);
        assert_eq!(mesh_data.normals.len(), 3);

        assert_eq!(mesh_data.positions, &[Vec3::ZERO, Vec3::X, Vec3::Y]);
        assert_eq!(mesh_data.indices, &[0, 1, 2]);
        let normal: [f32; 3] = Vec3::Z.into();
        assert_eq!(mesh_data.normals, &[normal, normal, normal]);
    }

    #[test]
    fn create_quad_works() {
        let mut mesh_data = MeshData::default();

        mesh_data.create_quad(&[Vec3::ZERO, Vec3::X, -Vec3::Y, Vec3::X - Vec3::Y]);

        assert_eq!(mesh_data.positions.len(), 4);
        assert_eq!(mesh_data.indices.len(), 6);
        assert_eq!(mesh_data.uvs.len(), 4);
        assert_eq!(mesh_data.normals.len(), 4);

        assert_eq!(
            mesh_data.positions,
            &[Vec3::ZERO, Vec3::X, -Vec3::Y, Vec3::X - Vec3::Y]
        );
        assert_eq!(mesh_data.indices, &[0, 2, 1, 1, 2, 3]);
        let normal: [f32; 3] = Vec3::Z.into();
        assert_eq!(mesh_data.normals, &[normal, normal, normal, normal]);
    }
}
