use yew::prelude::*;
use yew_autocomplete::{view::Plain, Autocomplete, ItemResolver, ItemResolverResult};
use yew_commons::FnProp;

use crate::COUNTRIES;

#[function_component(Simple)]
pub fn home() -> Html {
    let countries = use_state(|| Vec::new());

    let resolve_items: ItemResolver<String> =
        FnProp::from(|input: String| -> ItemResolverResult<String> {
            let items = COUNTRIES
                .into_iter()
                .filter(|s| s.to_lowercase().starts_with(input.to_lowercase().as_str()))
                .map(String::from)
                .collect();
            Box::pin(futures::future::ok::<_, ()>(items))
        });

    let onchange_single = {
        let countries = countries.clone();
        Callback::from(move |selected: Vec<String>| countries.set(selected.clone()))
    };

    html! {
        <>
            <h1>{"yew-commons: Autocomplete Demo"}</h1>
            <h2>{"multi_select: false, show_selected: false"}</h2>
            <div id={ "single-select" }>
                <p>{ if countries.is_empty() { html!{ "No countries has been selected."}} else { html!{ format!("Selected country: {}", countries.join(", ")) }} } </p>
                <Autocomplete<Plain, String>
                    onchange = { onchange_single }
                    resolve_items = { resolve_items.clone() }
                    show_selected = false
                    view = { Plain {} }
                />
            </div>
            <h2>{"multi_select: true, show_selected: true"}</h2>
            <div id={ "multi-select" }>
                <Autocomplete<Plain, String>
                    onchange = { Callback::from(|_| ()) }
                    multi_select = true
                    show_selected = true
                    {resolve_items}
                    view = { Plain {} }
                />
            </div>
        </>
    }
}
