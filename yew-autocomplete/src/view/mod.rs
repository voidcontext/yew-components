mod bulma;
mod plain;
mod render_html;

use std::rc::Rc;

use web_sys::{KeyboardEvent, MouseEvent};
use yew::Callback;

pub use bulma::Bulma;
pub use plain::Plain;
pub use render_html::RenderHtml;

#[derive(Clone, PartialEq)]
pub struct InputCallbacks {
    pub on_input: Callback<String>,
    pub on_keydown: Callback<KeyboardEvent>,
    pub resolve: Callback<MouseEvent>, // TODO: make this more generic
    pub select_item: Callback<usize>,
}

#[derive(Clone, PartialEq)]
pub struct Context<Item: Clone + PartialEq> {
    pub value: String,
    pub callbacks: InputCallbacks,
    pub items: Rc<Vec<Item>>,
    pub highlighted: Option<usize>,
    pub selected_items: Rc<Vec<Item>>,
    pub auto: bool,
}
