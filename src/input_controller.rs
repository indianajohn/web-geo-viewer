use super::{Model, Msg, ShouldRender};

pub fn update(model: &mut Model, msg: &Msg) -> ShouldRender {
    match msg {
        Msg::PointerDown(event) => {
            model.mouse_down = event.button();
        }
        Msg::PointerUp(_event) => {
            model.mouse_down = -1;
        }
        Msg::MouseUp(_event) => {
            model.mouse_down = -1;
        }
        Msg::TouchEnd(_event) => {
            model.mouse_down = -1;
        }
        Msg::PointerMove(event) => {
            if model.mouse_down != -1 {
                model.mouse_events.push(event.clone());
            }
        }
        Msg::PointerWheel(event) => {
            model.wheel_events.push(event.clone());
        }
        _ => {}
    }
    false
}
