use super::{Model, Msg, ShouldRender};
use yew::{html, Html};

pub fn update(model: &mut Model, msg: &Msg) -> ShouldRender {
    match msg {
        Msg::RemoveMesh(event) => {
            model.mesh.remove(event);
            return true;
        }
        Msg::MeshVisibilityToggle(event) => {
            let visible = &mut model.mesh.get_mut(event).unwrap().visible;
            *visible = !(*visible);
        }
        _ => {}
    };
    false
}
pub fn view_mesh_list(model: &Model) -> Html {
    html! {
        <table style="width:100%">
        { for model.mesh.iter().map(|f| view_element(model, f.0)) }
        </table>
    }
}

fn view_element(model: &Model, data: &str) -> Html {
    let mesh_name = data.to_string();
    let remove_mesh_cb = model
        .link
        .callback(move |_| Msg::RemoveMesh(mesh_name.clone()));
    let mesh_name = data.to_string();
    let handle_check_cb = model
        .link
        .callback(move |_| Msg::MeshVisibilityToggle(mesh_name.clone()));
    html! {
        <table>
            <tr>
            <td>
            { data }
            </td>
            <td>
            <input type="checkbox" checked={model.mesh.get(data).unwrap().visible } onclick=handle_check_cb />
            </td>
            <td>
            <button onclick=remove_mesh_cb.clone()>
                        { "Remove" }
            </button>
            </td>
            </tr>
        </table>
    }
}
