#[rustfmt::skip::macros(html)]
use yew::prelude::*;

use yew_autocomplete::yew_autocomplete::AutoComplete;

use wasm_bindgen::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{"yew-commons: Autocomplete Demo"}</h1>
            <AutoComplete />
        </>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<App>();

    Ok(())
}
