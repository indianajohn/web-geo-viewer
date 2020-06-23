#![recursion_limit = "256"]

use io::Material;
use std::collections::HashMap;
use three_d::{Camera, DeferredPipeline, Gl};
use web_sys::HtmlCanvasElement;
use yew::prelude::*;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::services::Task;
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

mod input_controller;
pub mod io;
mod mesh_list;
mod mesh_loader;
mod render_buffers;
mod render_canvas;
mod tests;
mod textured_mesh_renderer;
mod untextured_mesh_renderer;

pub struct MeshContainer {
    /// Vertices info.
    pub vertices: render_buffers::VertexData,

    /// Surfaces that reference these vertices
    pub surfaces: Vec<render_buffers::MeshSurface>,

    /// Show the mesh?
    pub visible: bool,
}

pub struct Model {
    canvas: Option<HtmlCanvasElement>,
    gl: Option<Gl>,
    camera: Option<Camera>,
    mesh: HashMap<String, MeshContainer>,
    materials: HashMap<String, Material>,
    images: HashMap<String, three_d::texture::Texture2D>,
    renderer: Option<DeferredPipeline>,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    render_loop: Option<Box<dyn Task>>,
    reader: ReaderService,
    tasks: Vec<ReaderTask>,
    mouse_down: i16,
    mouse_events: Vec<PointerEvent>,
    wheel_events: Vec<WheelEvent>,
    untextured_mesh_renderer: Option<untextured_mesh_renderer::UntexturedMeshRenderer>,
    textured_mesh_renderer: Option<textured_mesh_renderer::TexturedMeshRenderer>,
}

pub enum Msg {
    PointerDown(PointerEvent),
    PointerUp(PointerEvent),
    MouseUp(MouseEvent),
    TouchEnd(TouchEvent),
    PointerMove(PointerEvent),
    PointerWheel(WheelEvent),
    MeshVisibilityToggle(String),
    Render(f64),
    Loaded(FileData),
    RemoveMesh(String),
    Files(Vec<File>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            canvas: None,
            gl: None,
            camera: None,
            mesh: HashMap::new(),
            materials: HashMap::new(),
            images: HashMap::new(),
            renderer: None,
            link,
            node_ref: NodeRef::default(),
            render_loop: None,
            reader: ReaderService::new(),
            tasks: vec![],
            mouse_down: -1,
            mouse_events: vec![],
            wheel_events: vec![],
            untextured_mesh_renderer: None,
            textured_mesh_renderer: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        render_canvas::rendered(self, first_render)
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        let mut update = false;
        update = input_controller::update(self, &msg) || update;
        update = mesh_list::update(self, &msg) || update;
        update = render_canvas::update(self, &msg) || update;
        update = mesh_loader::update(self, &msg) || update;
        return update;
    }

    fn view(&self) -> Html {
        let down_cb = self
            .link
            .callback(|event: PointerEvent| Msg::PointerDown(event));
        let up_cb = self
            .link
            .callback(|event: PointerEvent| Msg::PointerUp(event));
        let touch_end_cb = self.link.callback(|event: TouchEvent| Msg::TouchEnd(event));
        let mouse_up_cb = self.link.callback(|event: MouseEvent| Msg::MouseUp(event));
        let move_cb = self
            .link
            .callback(|event: PointerEvent| Msg::PointerMove(event));
        let wheel_cb = self
            .link
            .callback(|event: WheelEvent| Msg::PointerWheel(event));
        html! {
            <div>
                <div>
                { mesh_loader::view(self) }
                </div>
                <table><tr>
                <td style="vertical-align: top">
                    <canvas ref={self.node_ref.clone()} onpointerdown=down_cb onpointerup=up_cb onpointermove=move_cb onmousewheel=wheel_cb onmouseout=mouse_up_cb ontouchend=touch_end_cb />
                </td>
                <td style="vertical-align: top">
                    { mesh_list::view_mesh_list(self) }
                </td>
                </tr></table>
                <p>{"An "}<a href="https://github.com/indianajohn/web-geo-viewer/" target="_blank">{"open source project"}</a>{". See "} <a href="https://github.com/indianajohn/web-geo-viewer/tree/master/docs" target="_blank">{"the docs"}</a>{" for more info."}</p>
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
