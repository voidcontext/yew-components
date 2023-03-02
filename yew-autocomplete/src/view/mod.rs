mod plain;
mod render_html;

use web_sys::InputEvent;
use yew::{prelude::Html, Callback};

pub use plain::Plain;
pub use render_html::RenderHtml;

pub trait View<Item> {
    fn input_field(&self, value: String, oninput: Callback<InputEvent>) -> Html;
    fn items(&self, items: &[Item]) -> Html;
}
