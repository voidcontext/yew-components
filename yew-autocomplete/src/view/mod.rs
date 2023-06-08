mod bulma;
mod plain;
mod render_html;

use std::rc::Rc;

use web_sys::{KeyboardEvent, MouseEvent};
use yew::{classes, html, Callback, Html};

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

pub(in crate::view) fn render_items<I: Clone + PartialEq + RenderHtml>(
    ctx: &Context<I>,
    additional_item_classes: &[&'static str],
    additional_highlighted_classes: &[&'static str],
) -> Vec<Html> {
    ctx.items
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let select_item = ctx.callbacks.select_item.clone();
            let onclick = Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                select_item.emit(index);
            });

            let mut classes = vec!["autocomplete-item"];
            classes.extend(additional_item_classes);

            if ctx.highlighted.iter().any(|h| *h == index) {
                classes.push("highlighted");
                classes.extend(additional_highlighted_classes);
            }

            html! { <a class={classes!(classes)} {onclick}>{value.render()}</a>}
        })
        .collect::<Vec<_>>()
}
