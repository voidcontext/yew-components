use std::ops::Deref;

use yew::prelude::*;
use yew_autocomplete::{
    view::{Bulma, Plain},
    Autocomplete, ItemResolver, ItemResolverResult,
};

use crate::{PageProps, View};

#[function_component(Issue001)]
pub fn issue_001(props: &PageProps) -> Html {
    let tags = use_state(Vec::<String>::new);

    let resolve_items: ItemResolver<String> = {
        let tags = tags.clone();
        Callback::from(move |input: String| -> ItemResolverResult<String> {
            let mut items = Vec::new();
            items.push(input.clone());
            let mut matching_tags: Vec<String> = tags
                .iter()
                .filter(|s| s.to_lowercase().starts_with(input.to_lowercase().as_str()))
                .map(String::from)
                .collect();
            items.append(&mut matching_tags);
            Box::pin(futures::future::ok::<_, ()>(items))
        })
    };

    let onchange_single = {
        let state = tags.clone();
        Callback::from(move |selected: Vec<String>| {
            let mut tags = state.deref().clone();
            tags.push(selected[0].clone());
            tags.sort();
            state.set(tags);
        })
    };

    let view = match props.view {
        View::Plain => html! { <Plain<String> /> },
        View::Bulma => html! { <Bulma<String> /> },
    };

    html! {
        <>
            <h1 class="title">{"yew-components: Autocomplete Demo"}</h1>
            <h2 class="subtitle">{"multi_select: false, show_selected: false"}</h2>
            <div id={ "single-select" }>
                <p class="block">{ if tags.is_empty() { html!{ "No tags has been created."}} else { html!{ format!("Tags: {}", tags.join(", ")) }} } </p>
                <Autocomplete<String>
                    onchange = { onchange_single }
                    {resolve_items}
                >
                    {view}
                </Autocomplete<String>>
            </div>
        </>
    }
}
