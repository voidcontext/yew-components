mod plain;
mod render_html;

use std::rc::Rc;

use web_sys::{InputEvent, KeyboardEvent};
use yew::Callback;

pub use plain::Plain;
pub use render_html::RenderHtml;

#[derive(Clone, PartialEq)]
pub struct InputCallbacks {
    pub on_input: Callback<InputEvent>,
    pub on_keydown: Callback<KeyboardEvent>,
}

#[derive(Clone, PartialEq)]
pub struct Context<Item: Clone + PartialEq> {
    pub value: String,
    pub callbacks: InputCallbacks,
    pub items: Rc<Vec<Item>>,
    pub highlighted: Option<usize>,
    pub selected_items: Rc<Vec<Item>>,
}
