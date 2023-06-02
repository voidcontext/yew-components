mod bulma;
mod plain;
mod render_html;

use std::rc::Rc;

use web_sys::KeyboardEvent;
use yew::Callback;

pub use bulma::Bulma;
pub use plain::Plain;
pub use render_html::RenderHtml;

use crate::Config;

#[derive(Clone, PartialEq)]
pub struct InputCallbacks {
    pub on_input: Callback<String>,
    pub on_keydown: Callback<KeyboardEvent>,
}

#[derive(Clone, PartialEq)]
pub struct Context<Item: Clone + PartialEq> {
    pub value: String,
    pub callbacks: InputCallbacks,
    pub items: Rc<Vec<Item>>,
    pub highlighted: Option<usize>,
    pub selected_items: Rc<Vec<Item>>,
    pub config: Config,
}
