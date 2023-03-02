#[rustfmt::skip::macros(html)]
use yew::prelude::*;

use yew_autocomplete::{view::Plain, Autocomplete, ItemResolver, ItemResolverResult};
use yew_commons::fn_prop::FnProp;

use wasm_bindgen::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let resolve_items: ItemResolver<String> =
        FnProp::from(|input: String| -> ItemResolverResult<String> {
            let items = vec!["bar", "foo", "foobar"]
                .into_iter()
                .filter(|s| s.starts_with(input.as_str()))
                .map(String::from)
                .collect();
            Box::pin(futures::future::ok::<_, ()>(items))
        });
    let view = Plain {};

    html! {
        <>
            <h1>{"yew-commons: Autocomplete Demo"}</h1>
            <Autocomplete<Plain, String>
                {resolve_items}
                {view}
            />
        </>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::Renderer::<App>::new().render();

    Ok(())
}
