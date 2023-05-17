use yew::prelude::*;
use yew_autocomplete::{view::Plain, Autocomplete, Config, ItemResolver, ItemResolverResult};
use yew_commons::FnProp;

use crate::COUNTRIES;

#[function_component(Simple)]
pub fn simple() -> Html {
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

    let config = Config::default();

    html! {
        <>
            <h1>{"yew-commons: Autocomplete Demo"}</h1>
            <h2>{"multi_select: false, show_selected: false"}</h2>
            <div id={ "single-select" }>
                <p>{ if countries.is_empty() { html!{ "No countries has been selected."}} else { html!{ format!("Selected country: {}", countries.join(", ")) }} } </p>
                <Autocomplete<String>
                    onchange = { onchange_single }
                    {resolve_items}
                    {config}
                >
                    <Plain<String> />
                </Autocomplete<String>>
            </div>
        </>
    }
}
