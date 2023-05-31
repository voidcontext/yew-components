use std::{fmt::Display, str::FromStr};

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
    #[at("/:view/simple")]
    Simple { view: View },
    #[at("/:view/multi")]
    Multi,
    #[at("/:view/non-auto")]
    NonAuto,
}

#[derive(Clone, PartialEq)]
pub enum View {
    Plain,
}

#[derive(Properties, PartialEq)]
pub struct PageProps {
    view: View,
}

impl Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            View::Plain => "plain",
        })
    }
}

impl FromStr for View {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(View::Plain),
            _ => Err(format!("Invalid view value {}", s)),
        }
    }
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
        Route::Simple { view } => html! { <simple::Simple {view} /> },
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
