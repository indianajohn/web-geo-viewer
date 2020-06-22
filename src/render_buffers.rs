use three_d::objects::Error;
use three_d::*;

/// All of the information needed to render a mesh, in the most convenient
/// form possible for rendering.

/// Everything associated with a surface.
pub struct MeshSurface {
    /// Indices for triangular faces. A 1D int array with a
    /// stride of 3.
    pub index_buffer: ElementBuffer,

    /// UVs. a 1D float array with a stride of 2.
    pub maybe_uvs: Option<VertexBuffer>,

    /// The name of the material, if there is one.
    pub maybe_material_name: Option<String>,
}

/// Everything associated with vertices. We separate vertex data from
/// index, material, and texture data because several "materials" might
/// belong to the same vertex list, and at render time we would have
/// a shader point to the same vertex buffer for multiple render passes
/// with different index vectors, etc...
pub struct VertexData {
    /// Positions. a 1D array with a stride of 3.
    pub position_buffer: VertexBuffer,

    /// per-vertex normals
    pub normal_buffer: VertexBuffer,
}

impl VertexData {
    pub fn new(gl: &Gl, positions: &[f32], normals: &[f32]) -> Result<Self, Error> {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, positions)?;
        let normal_buffer = VertexBuffer::new_with_static_f32(gl, normals)?;

        Ok(VertexData {
            position_buffer,
            normal_buffer,
        })
    }
}

impl MeshSurface {
    pub fn new(
        gl: &Gl,
        indices: &[u32],
        maybe_uvs_cpu: Option<&[f32]>,
        maybe_material_name: Option<String>,
    ) -> Result<Self, Error> {
        let index_buffer = ElementBuffer::new_with_u32(gl, indices)?;
        let mut maybe_uvs_gpu: Option<VertexBuffer> = None;
        match maybe_uvs_cpu {
            Some(uvs_cpu) => {
                let uvs_gpu = VertexBuffer::new_with_static_f32(gl, uvs_cpu)?;
                maybe_uvs_gpu = Some(uvs_gpu);
            }
            None => {}
        }

        Ok(MeshSurface {
            index_buffer: index_buffer,
            maybe_uvs: maybe_uvs_gpu,
            maybe_material_name: maybe_material_name,
        })
    }
}
