use crate::io::Material;
use crate::render_buffers::{MeshSurface, VertexData};
use three_d::*;

pub struct UntexturedMeshRenderer {
    shader: program::Program,
}

impl UntexturedMeshRenderer {
    pub fn new(gl: &Gl) -> UntexturedMeshRenderer {
        UntexturedMeshRenderer {
            shader: program::Program::from_source(
                &gl,
                include_str!("shaders/mesh_shaded.vert"),
                include_str!("shaders/shaded.frag"),
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
            .add_uniform_vec3("color", &material.diffuse_color)
            .unwrap();
        program
            .add_uniform_mat4("modelMatrix", &transformation)
            .unwrap();
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
        program.draw_elements(&mesh_surface.index_buffer);
    }
}
