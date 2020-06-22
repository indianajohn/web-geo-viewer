use crate::io::Material;
use crate::render_buffers::{MeshSurface, VertexData};
use three_d::*;

pub struct TexturedMeshRenderer {
    shader: program::Program,
}

impl TexturedMeshRenderer {
    pub fn new(gl: &Gl) -> TexturedMeshRenderer {
        TexturedMeshRenderer {
            shader: program::Program::from_source(
                &gl,
                include_str!("shaders/textured.vert"),
                include_str!("shaders/textured.frag"),
            )
            .unwrap(),
        }
    }

    pub fn render(
        &self,
        transformation: &Mat4,
        camera: &camera::Camera,
        vertex_data: &VertexData,
        mesh_surface: &MeshSurface,
        material: &Material,
        image: &Texture2D,
    ) {
        let program = &self.shader;
        program
            .add_uniform_float("diffuse_intensity", &material.diffuse_intensity)
            .unwrap();
        program
            .add_uniform_float("specular_intensity", &material.specular_intensity)
            .unwrap();
        let specular_power = 5.0;
        program
            .add_uniform_float("specular_power", &specular_power)
            .unwrap();

        program
            .add_uniform_mat4("modelMatrix", &transformation)
            .unwrap();

        program.use_texture(image, "texture0").unwrap();

        program.use_uniform_block(camera.matrix_buffer(), "Camera");
        program
            .add_uniform_mat4(
                "normalMatrix",
                &transformation.invert().unwrap().transpose(),
            )
            .unwrap();
        program
            .use_attribute_vec3_float(&vertex_data.position_buffer, "position")
            .unwrap();
        program
            .use_attribute_vec3_float(&vertex_data.normal_buffer, "normal")
            .unwrap();
        program
            .use_attribute_vec3_float(mesh_surface.maybe_uvs.as_ref().unwrap(), "uvw")
            .unwrap();
        program.draw_elements(&mesh_surface.index_buffer);
    }
}
