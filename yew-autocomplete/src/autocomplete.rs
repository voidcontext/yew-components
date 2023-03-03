use std::{future::Future, marker::PhantomData, pin::Pin};

use web_sys::HtmlInputElement;
use yew::{html::Scope, prelude::*};
use yew_commons::fn_prop::FnProp;

use crate::{
    autocomplete_state::AutocompleteState,
    view::{RenderHtml, View},
};

pub fn make_callback<M, C, E: AsRef<Event>, F: Fn(String) -> M + 'static>(
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

/// The async result of the `ItemResolver`
pub type ItemResolverResult<T> = Pin<Box<dyn Future<Output = Result<Vec<T>, ()>>>>;

/// `ItemResolver` is an async function that can be passed as a Prop, that takes the current value of
/// the `Autocomplete` input and returns a Vec of Ts
pub type ItemResolver<T> = FnProp<String, ItemResolverResult<T>>;

pub struct Autocomplete<V: View<T>, T> {
    state: AutocompleteState<T>,
    _view: PhantomData<V>,
}

#[derive(PartialEq, Properties)]
pub struct Props<V: View<T> + PartialEq, T: PartialEq> {
    pub resolve_items: ItemResolver<T>,
    pub view: V,
}

#[derive(Debug, PartialEq)]
pub enum Msg<T> {
    OnInput(String),
    SetItems(Vec<T>),
}

impl<V: 'static + View<T> + PartialEq, T: PartialEq + Clone + RenderHtml + 'static> Component
    for Autocomplete<V, T>
{
    type Message = Msg<T>;

    type Properties = Props<V, T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: AutocompleteState::default(),
            _view: PhantomData::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnInput(value) => {
                self.state.oninput(
                    value.as_str(),
                    |f| ctx.link().send_future(f),
                    ctx.props().resolve_items.clone(),
                );
                true
            }
            Msg::SetItems(items) => {
                self.state.set_items(items);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let view = &ctx.props().view;

        let input_field = view.input_field(self.state.input(), make_callback(link, Msg::OnInput));

        let items = view.items(&self.state.items());

        html! {
            <>
                {input_field}
                {items}
            </>
        }
    }
}
