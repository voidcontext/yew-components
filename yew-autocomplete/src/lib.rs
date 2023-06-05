//! This crate provides a highly configurable auto-completion component

mod autocomplete;
mod autocomplete_state;
mod config;
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
