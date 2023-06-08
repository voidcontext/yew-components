use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Html;

use crate::render_if;

use super::render_items;
use super::RenderHtml;

#[function_component(Plain)]
pub fn plain<T: 'static + Clone + PartialEq + RenderHtml>() -> Html {
    let view_ctx = use_context::<super::Context<T>>().expect("view::Context wasn't provided");

    let items = render_items(&view_ctx, &[], &[])
        .into_iter()
        .map(|item| {
            html! { <li>{item}</li>}
        })
        .collect::<Html>();
    let selected_lis = view_ctx
        .selected_items
        .iter()
        .map(|value| {
            html! { <li class={classes!("autocomplete-item", "selected")}>{value.render()}</li> }
        })
        .collect::<Html>();

    let input_cb = view_ctx.callbacks.on_input.clone();
    let oninput = move |e: InputEvent| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        input_cb.emit(input.value());
    };
    let onsearch = view_ctx.callbacks.resolve.clone();

    html! {
        <div>
            {
                render_if(!view_ctx.selected_items.is_empty(), html!{
                    <ul class="selected-items">
                        { selected_lis }
                    </ul>
                })
            }
            <input
                type="text"
                value={view_ctx.value.clone()}
                {oninput}
                onkeydown={view_ctx.callbacks.on_keydown.clone()}
            />
            {
                render_if(
                    !view_ctx.auto,
                    html! {
                        <input type="button" value="Search" onclick={onsearch}/>
                    }
                )
            }
            {
                render_if(!view_ctx.items.is_empty(), html!{
                    <ul class="autocomplete-items">
                        { items }
                    </ul>
                })
            }
        </div>
    }
}
