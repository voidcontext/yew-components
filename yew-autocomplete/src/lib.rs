//! This crate provides a highly configurable auto-completion component
//!
//! A minimal example that showcases the main building blocks:
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_autocomplete::{view::Bulma, Autocomplete, ItemResolver, ItemResolverResult};
//! use yew_commons::FnProp;
//!
//! let onchange = |_: Vec<String>| ();
//! let resolve_items: ItemResolver<String> =
//!     FnProp::from(|_: String| -> ItemResolverResult<String>  {
//!         Box::pin(async { Ok(Vec::<String>::new()) })
//!     });
//!
//! html! {
//!     <Autocomplete<String>
//!         {resolve_items}
//!         {onchange}
//!         auto = false
//!     >
//!         <Bulma<String> />
//!     </Autocomplete<String>>
//! };
//! ```

mod autocomplete;
mod autocomplete_state;
pub mod view;

pub use autocomplete::*;
use web_sys::{Event, HtmlInputElement};
use yew::{html::Scope, Callback, Component, TargetCast};

pub(crate) fn make_callback<M, C, E: AsRef<Event>, F: Fn(String) -> M + 'static>(
    link: &Scope<C>,
    f: F,
) -> Callback<E>
where
    C: Component<Message = M>,
{
    link.callback(move |e: E| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        f(input.value())
    })
}
