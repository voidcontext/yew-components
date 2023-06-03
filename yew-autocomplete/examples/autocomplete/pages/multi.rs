use yew::prelude::*;
use yew_autocomplete::{
    view::{Bulma, Plain},
    Autocomplete, Config, ItemResolver, ItemResolverResult,
};
use yew_commons::FnProp;

use crate::{PageProps, View, COUNTRIES};

#[function_component(Multi)]
pub fn multi(props: &PageProps) -> Html {
    let resolve_items: ItemResolver<String> =
        FnProp::from(|input: String| -> ItemResolverResult<String> {
            let items = COUNTRIES
                .into_iter()
                .filter(|s| s.to_lowercase().starts_with(input.to_lowercase().as_str()))
                .map(String::from)
                .collect();
            Box::pin(futures::future::ok::<_, ()>(items))
        });

    let config = Config {
        multi_select: true,
        show_selected: true,
        ..Config::default()
    };

    let view = match props.view {
        View::Plain => html! { <Plain<String> /> },
        View::Bulma => html! { <Bulma<String> /> },
    };

    html! {
        <>
            <h1 class="title">{"yew-components: Autocomplete Demo"}</h1>
            <h2 class="subtitle">{"multi_select: true, show_selected: true"}</h2>
            <div id={ "multi-select" }>
                <Autocomplete<String>
                    onchange = { Callback::from(|_| ()) }
                    {config}
                    {resolve_items}
                >
                    {view}
                </Autocomplete<String>>
            </div>
        </>
    }
}
