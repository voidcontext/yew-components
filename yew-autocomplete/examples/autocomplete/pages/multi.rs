use yew::prelude::*;
use yew_autocomplete::{view::Plain, Autocomplete, ItemResolver, ItemResolverResult};
use yew_commons::FnProp;

use crate::COUNTRIES;

#[function_component(Multi)]
pub fn multi() -> Html {
    let resolve_items: ItemResolver<String> =
        FnProp::from(|input: String| -> ItemResolverResult<String> {
            let items = COUNTRIES
                .into_iter()
                .filter(|s| s.to_lowercase().starts_with(input.to_lowercase().as_str()))
                .map(String::from)
                .collect();
            Box::pin(futures::future::ok::<_, ()>(items))
        });

    html! {
        <>
            <h1>{"yew-commons: Autocomplete Demo"}</h1>
            <h2>{"multi_select: true, show_selected: true"}</h2>
            <div id={ "multi-select" }>
                <Autocomplete<String>
                    onchange = { Callback::from(|_| ()) }
                    multi_select = true
                    show_selected = true
                    {resolve_items}
                >
                    <Plain<String> />
                </Autocomplete<String>>
            </div>
        </>
    }
}
