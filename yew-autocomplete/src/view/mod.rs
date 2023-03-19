mod plain;
mod render_html;

use web_sys::{InputEvent, KeyboardEvent};
use yew::{prelude::Html, Callback};

pub use plain::Plain;
pub use render_html::RenderHtml;

pub struct InputCallbacks {
    pub on_input: Callback<InputEvent>,
    pub on_keydown: Callback<KeyboardEvent>,
}

pub trait View<Item> {
    fn input_field(&self, value: String, callbacks: InputCallbacks) -> Html;
    fn items(&self, items: &[Item], highlighted: &Option<usize>) -> Html;
}
