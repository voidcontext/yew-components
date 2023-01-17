use yew::prelude::*;

use wasm_bindgen::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{"yew-commons: Autocomplete Demo"}</h1>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<App>();

    Ok(())
}
