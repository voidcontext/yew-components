#[rustfmt::skip::macros(html)]
use yew::prelude::*;

use yew_router::prelude::*;

use wasm_bindgen::prelude::*;

use pages::simple::Simple;

pub use data::countries::COUNTRIES;

mod data;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/simple")]
    Simple,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <>
            <h1>{"yew-commons: Autocomplete Demo"}</h1>
            <a href="/simple">{"simple"}</a>
            </>
        },
        Route::Simple => html! { <Simple />},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::Renderer::<App>::new().render();

    Ok(())
}
