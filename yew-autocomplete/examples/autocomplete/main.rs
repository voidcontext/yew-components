#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::let_underscore_untyped)]

use std::{fmt::Display, str::FromStr};

use gloo_utils::document;
use wasm_bindgen::prelude::*;
#[rustfmt::skip::macros(html)]
use yew::prelude::*;
use yew_router::prelude::*;

pub use data::countries::COUNTRIES;
use pages::{issue_001, multi, non_auto, simple};

mod data;
mod pages;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/:view/simple")]
    Simple { view: View },
    #[at("/:view/multi")]
    Multi { view: View },
    #[at("/:view/nonauto")]
    NonAuto { view: View },
    #[at("/:view/issue-001")]
    Issue001 { view: View },
}

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Plain,
    Bulma,
}

#[derive(Debug, Properties, PartialEq)]
pub struct PageProps {
    view: View,
}

impl Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            View::Plain => "plain",
            View::Bulma => "bulma",
        })
    }
}

impl FromStr for View {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(View::Plain),
            "bulma" => Ok(View::Bulma),
            _ => Err(format!("Invalid view value {s}")),
        }
    }
}

fn switch(route: Route) -> Html {
    // log(&format!("route: {route:?}"));
    match route {
        Route::Home => html! {
            <Redirect<Route> to={Route::Simple { view: View::Plain }}/>
        },
        Route::Simple { view } => html! {
            <Tabs example="Simple" view={view.clone()}><simple::Simple {view} /></Tabs>
        },
        Route::Multi { view } => html! {
            <Tabs example="Multi" view={view.clone()}><multi::Multi {view} /></Tabs>

        },
        Route::NonAuto { view } => html! {
            <Tabs example="NonAuto" view={view.clone()}><non_auto::NonAuto {view} /></Tabs>
        },
        Route::Issue001 { view } => html! {
            <issue_001::Issue001 {view} />
        },
    }
}

#[derive(Properties, PartialEq)]
struct TabsProps {
    children: Children,
    example: String,
    view: View,
}

#[function_component(Tabs)]
fn tabs(props: &TabsProps) -> Html {
    let examples = ["Simple", "Multi", "NonAuto"];
    let views = [&View::Plain, &View::Bulma];

    let mut tabs = Vec::new();

    for example in examples {
        for view in views {
            let mut classes = Vec::new();

            if example == props.example && view == &props.view {
                classes.push("is-active");
            }

            let mut tag_classes = vec!["tag", "is-light"];
            if view.to_string().as_str() == "bulma" {
                tag_classes.push("is-primary");
            }
            tabs.push(
                html! {
                    <li class={classes!(classes)}>
                        <a href={format!("/{}/{}", view, example.to_lowercase()) }>
                            {example}
                            <span class={classes!(tag_classes)} style="margin-left: 0.5rem">{format!("view: {view}")}</span>
                        </a>
                    </li>
                }
            );
        }
    }

    html! {
        <>
        <div class="tabs">
            <ul>
                { for tabs  }
            </ul>
        </div>
         {for props.children.iter() }
        </>
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
    yew::Renderer::<App>::with_root(document().get_element_by_id("app").unwrap()).render();

    Ok(())
}
