use yew::prelude::*;
use yew::{Html, Properties};

use super::{InputCallbacks, RenderHtml, View};

#[derive(PartialEq, Properties)]
pub struct Plain;

impl<T: RenderHtml> View<T> for Plain {
    fn input_field(&self, value: String, callbacks: InputCallbacks) -> yew::Html {
        html! {
            <input
                type="text"
                {value}
                oninput={callbacks.on_input}
                onkeydown={callbacks.on_keydown}
            />
        }
    }

    fn items(&self, items: &[T], highlighed: &Option<usize>) -> yew::Html {
        let lis = items
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let mut classes = vec!["autocomplete-item"];

                if highlighed.iter().any(|h| *h == index) {
                    classes.push("highlighted");
                }

                html! { <li class={classes!(classes)}>{value.render()}</li>}
            })
            .collect::<Html>();

        html! {
            <ul>
                { lis }
            </ul>
        }
    }

    fn selected_items(&self, items: &[T]) -> yew::Html {
        let lis = items
            .iter()
            .map(|value| {
                html! { <li class={classes!("autocomplete-item", "selected")}>{value.render()}</li>}
            })
            .collect::<Html>();

        html! {
            <ul>
                { lis }
            </ul>
        }
    }
}
