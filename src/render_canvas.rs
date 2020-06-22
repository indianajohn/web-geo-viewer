use super::{Model, Msg, ShouldRender};
use crate::io::Material;
use crate::textured_mesh_renderer::TexturedMeshRenderer;
use crate::untextured_mesh_renderer::UntexturedMeshRenderer;
use three_d::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::WebGl2RenderingContext as GL;
use yew::services::RenderService;

static ZOOM_SPEED: f32 = 0.01f32;
static MOVE_SPEED: f32 = 0.01f32;

pub fn update(model: &mut Model, msg: &Msg) -> ShouldRender {
    match msg {
        Msg::Render(timestamp) => {
            render_gl(model, *timestamp);
        }
        _ => {}
    };
    false
}

fn render_gl(model: &mut Model, _timestamp: f64) {
    let gl = model.gl.as_ref().expect("GL Context not initialized!");
    let screen_width = model.canvas.as_ref().unwrap().width();
    let screen_height = model.canvas.as_ref().unwrap().height();
    model
        .camera
        .as_mut()
        .unwrap()
        .set_size(screen_width as f32, screen_height as f32);

    for mouse_event in model.mouse_events.iter() {
        if mouse_event.shift_key() || model.mouse_down == 2 {
            let target = model.camera.as_ref().unwrap().target();
            let position = model.camera.as_ref().unwrap().position();
            let up = model.camera.as_ref().unwrap().up();
            let forward = target - position;
            let right = forward.cross(*up);
            let translation_y = MOVE_SPEED * (mouse_event.movement_y() as f32) * up;
            let translation_x = -1f32 * MOVE_SPEED * (mouse_event.movement_x() as f32) * right;
            let translation = translation_x + translation_y;
            model.camera.as_mut().unwrap().translate(&translation);
        } else {
            model.camera.as_mut().unwrap().rotate(
                mouse_event.movement_x() as f32,
                mouse_event.movement_y() as f32,
            );
        }
    }
    model.mouse_events = vec![];
    for wheel_event in model.wheel_events.iter() {
        model
            .camera
            .as_mut()
            .unwrap()
            .zoom(ZOOM_SPEED * wheel_event.delta_y() as f32);
    }
    model.wheel_events = vec![];

    let ambient_light0 = AmbientLight::new(&gl, 0.2, &vec3(0.8, 0.8, 0.8)).unwrap();
    let directional_light0 =
        DirectionalLight::new(&gl, 1.0, &vec3(0.8, 0.8, 0.8), &vec3(0.0, -1.0, 0.0)).unwrap();
    let directional_light1 =
        DirectionalLight::new(&gl, 1.0, &vec3(0.8, 0.8, 0.8), &vec3(0.0, 1.0, 0.0)).unwrap();
    let point_light0 = PointLight::new(
        &gl,
        0.8,
        &vec3(0.8, 0.8, 0.8),
        &vec3(-5.0, 0.0, 0.0),
        0.5,
        0.05,
        0.005,
    )
    .unwrap();
    let point_light1 = PointLight::new(
        &gl,
        0.8,
        &vec3(0.8, 0.8, 0.8),
        &vec3(5.0, 0.0, 0.0),
        0.5,
        0.05,
        0.005,
    )
    .unwrap();
    let renderer = model.renderer.as_mut().unwrap();
    let camera = model.camera.as_ref().unwrap();
    let mesh_groups = &model.mesh;
    let materials = &model.materials;
    let images = &model.images;
    let untextured = model.untextured_mesh_renderer.as_ref();
    let textured = model.textured_mesh_renderer.as_ref();
    renderer
        .geometry_pass(screen_width as usize, screen_height as usize, &|| {
            for group in mesh_groups.iter() {
                for surface in group.1.surfaces.iter() {
                    if group.1.visible {
                        let mut maybe_texture: Option<&Texture2D> = None;
                        let material = match &surface.maybe_material_name {
                            Some(material_name) => match materials.get(material_name) {
                                Some(material) => {
                                    if let Some(texture_name) = &material.texture_name {
                                        if let Some(texture) = images.get(texture_name) {
                                            maybe_texture = Some(texture);
                                        }
                                    }
                                    material.clone()
                                }
                                None => Material::new(),
                            },
                            None => Material::new(),
                        };
                        match &surface.maybe_uvs {
                            Some(_) => match maybe_texture {
                                Some(texture) => textured.unwrap().render(
                                    &Mat4::identity(),
                                    camera,
                                    &group.1.vertices,
                                    surface,
                                    &material,
                                    &texture,
                                ),
                                None => untextured.unwrap().render(
                                    &Mat4::identity(),
                                    camera,
                                    &group.1.vertices,
                                    surface,
                                    &material,
                                ),
                            },
                            None => untextured.unwrap().render(
                                &Mat4::identity(),
                                camera,
                                &group.1.vertices,
                                surface,
                                &material,
                            ),
                        }
                    }
                }
            }
        })
        .unwrap();
    Screen::write(
        &gl,
        0,
        0,
        screen_width as usize,
        screen_height as usize,
        Some(&vec4(0.1, 0.1, 0.1, 1.0)),
        None,
        &|| {
            model
                .renderer
                .as_ref()
                .unwrap()
                .light_pass(
                    &model.camera.as_ref().unwrap(),
                    Some(&ambient_light0),
                    &[&directional_light0, &directional_light1],
                    &[],
                    &[&point_light0, &point_light1],
                )
                .unwrap();
        },
    )
    .unwrap();

    let render_frame = model.link.callback(Msg::Render);
    let handle = RenderService::new().request_animation_frame(render_frame);

    // A reference to the new handle must be retained for the next render to run.
    model.render_loop = Some(Box::new(handle));
}

/// Not implemented for non-wasm. In place so tests can be run on
/// the host system.
#[cfg(not(target_arch = "wasm32"))]
pub fn new_gl(_gl: &GL) -> Option<Gl> {
    None
}

#[cfg(target_arch = "wasm32")]
pub fn new_gl(gl: &GL) -> Option<Gl> {
    gl::Glstruct::new(gl.clone()).into()
}

pub fn rendered(model: &mut Model, first_render: bool) {
    let canvas = model.node_ref.cast::<HtmlCanvasElement>().unwrap();

    let gl: GL = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    model.canvas = Some(canvas);
    model.gl = new_gl(&gl);
    let gl_ref = model.gl.as_ref().expect("GL Context not initialized!");

    model.renderer = Some(DeferredPipeline::new(&gl_ref).unwrap());
    model.untextured_mesh_renderer = Some(UntexturedMeshRenderer::new(&gl_ref));
    model.textured_mesh_renderer = Some(TexturedMeshRenderer::new(&gl_ref));

    // Camera
    let camera = Camera::new_perspective(
        &gl_ref,
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        model.canvas.as_ref().unwrap().width() as f32
            / model.canvas.as_ref().unwrap().width() as f32,
        0.1,
        150.0,
    );
    model.camera = Some(camera);

    if first_render {
        // The callback to request animation frame is passed a time value which can be used for
        // rendering motion independent of the framerate which may vary.
        let render_frame = model.link.callback(Msg::Render);
        let handle = RenderService::new().request_animation_frame(render_frame);
        let available_x = web_sys::window()
            .unwrap()
            .screen()
            .unwrap()
            .avail_width()
            .unwrap();
        let available_y = web_sys::window()
            .unwrap()
            .screen()
            .unwrap()
            .avail_height()
            .unwrap();
        let desired_x = (available_x as f32) * 0.8;
        model.canvas.as_mut().unwrap().set_width(desired_x as u32);
        let desired_y = (available_y as f32) * 0.8;
        model.canvas.as_mut().unwrap().set_height(desired_y as u32);

        // A reference to the handle must be stored, otherwise it is dropped and the render won't
        // occur.
        model.render_loop = Some(Box::new(handle));

        // Disable context menu
        model
            .canvas
            .as_mut()
            .unwrap()
            .set_attribute("oncontextmenu", "event.preventDefault();")
            .unwrap();
    }
}
