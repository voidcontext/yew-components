use web_sys::InputEvent;
use yew::prelude::html;
use yew::{Callback, Html, Properties};

use super::{RenderHtml, View};

#[derive(PartialEq, Properties)]
pub struct Plain;

impl<T: RenderHtml> View<T> for Plain {
    fn input_field(&self, value: String, oninput: Callback<InputEvent>) -> yew::Html {
        html! {
            <input type="text" {value} {oninput} />
        }
    }

    fn items(&self, items: &[T]) -> yew::Html {
        let lis = items
            .iter()
            .map(|value| html! { <li class="autocomplete-item">{value.render()}</li>})
            .collect::<Html>();

        html! {
            <ul>
                { lis }
            </ul>
        }
    }
}
