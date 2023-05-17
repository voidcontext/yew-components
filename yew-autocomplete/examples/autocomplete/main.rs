use wasm_bindgen::prelude::*;
#[rustfmt::skip::macros(html)]
use yew::prelude::*;
use yew_router::prelude::*;

pub use data::countries::COUNTRIES;
use pages::*;

mod data;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/simple")]
    Simple,
    #[at("/multi")]
    Multi,
    #[at("/non-auto")]
    NonAuto,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <>
            <h1>{"yew-commons: Autocomplete Demo"}</h1>
            <div>
                <a href="/simple">{"simple"}</a>
            </div>
            <div>
                <a href="/multi">{"multi"}</a>
            </div>
            <div>
                <a href="/non-auto">{"non-auto"}</a>
            </div>
            </>
        },
        Route::Simple => html! { <simple::Simple /> },
        Route::Multi => html! { <multi::Multi/> },
        Route::NonAuto => html! { <non_auto::NonAuto/> },
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
