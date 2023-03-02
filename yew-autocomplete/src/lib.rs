use std::{future::Future, marker::PhantomData, pin::Pin};

use autocomplete_state::AutocompleteState;
use view::{RenderHtml, View};
use web_sys::HtmlInputElement;
use yew::{html::Scope, prelude::*};
use yew_commons::fn_prop::FnProp;

mod autocomplete_state;
pub mod view;

pub fn make_callback<M, C, E: AsRef<Event>, F: 'static>(link: &Scope<C>, f: F) -> Callback<E>
where
    C: Component<Message = M>,
    F: Fn(String) -> M,
{
    link.callback(move |e: E| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        f(input.value())
    })
}

/// The async result of the ItemResolver
pub type ItemResolverResult<T> = Pin<Box<dyn Future<Output = Result<Vec<T>, ()>>>>;

/// ItemResolver is an async function that can be passed as a Prop, that takes the current value of
/// the Autocomplete input and returns a Vec of Ts
pub type ItemResolver<T> = FnProp<String, ItemResolverResult<T>>;

pub struct Autocomplete<V: View<T>, T> {
    state: AutocompleteState<T>,
    _view: PhantomData<V>,
}

#[derive(PartialEq, Properties)]
pub struct AutocompleteProps<V: View<T> + PartialEq, T: PartialEq> {
    pub resolve_items: ItemResolver<T>, // TODO: refactor String to <T: RenderHtml>
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

    type Properties = AutocompleteProps<V, T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: Default::default(),
            _view: Default::default(),
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
        let input_field = ctx
            .props()
            .view
            .input_field(self.state.input(), make_callback(ctx.link(), Msg::OnInput));

        let items = ctx.props().view.items(&self.state.items());

        html! {
            <>
                {input_field}
                {items}
            </>
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn pass() {
        assert!(true)
    }
}
