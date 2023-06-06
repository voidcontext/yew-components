use std::{future::Future, pin::Pin, rc::Rc};

use yew::{html::Scope, prelude::*};
use yew_commons::fn_prop::FnProp;

use crate::{
    autocomplete_state::{AutocompleteState, HighlightDirection},
    view::{self, InputCallbacks, RenderHtml},
};

/// The async result of the [`ItemResolver`]
pub type ItemResolverResult<T> = Pin<Box<dyn Future<Output = Result<Vec<T>, ()>>>>;

/// An async function that can be passed as a Prop, that takes the current value of the
/// [`Autocomplete`] input and returns a Vec of Ts
pub type ItemResolver<T> = FnProp<String, ItemResolverResult<T>>;

/// A Yew.rs [Component] with highly configurable auto completion capabilites
pub struct Autocomplete<T: Clone + PartialEq + RenderHtml + 'static> {
    state: AutocompleteState<T, AsyncMessageCallback<Self>>,
}

/// Properties of the [Autocomplete] component
#[derive(PartialEq, Properties, Clone)]
pub struct Props<T: PartialEq> {
    pub resolve_items: ItemResolver<T>,
    pub onchange: Callback<Vec<T>>,
    pub children: Children, // TODO: typed children?

    #[prop_or(true)]
    pub auto: bool,
    #[prop_or(false)]
    pub show_selected: bool,
    #[prop_or(false)]
    pub multi_select: bool,
}

/// Internal messages of the [Autocomplete] component
#[derive(Debug, PartialEq)]
pub enum Msg<T> {
    OnInput(String),
    OnKeydown(u32),
    SetItems(Vec<T>),
    SelectItem(usize),
    Noop,
}

/// An synchronously executed async callback
pub(crate) trait AsyncCallback<T> {
    /// Synchronously dispatches  the given future
    fn dispatch(&self, future: Pin<Box<dyn Future<Output = T>>>);
}

/// An async callback that is capable of sending an async message (a [`Component::Message`] wrapped in a Future) to
/// a component
struct AsyncMessageCallback<C: Component> {
    link: Scope<C>,
}

impl<C: Component> AsyncMessageCallback<C> {
    /// Creates a new [`AsyncMessageCallback`] from the [Scope] of a [Component]
    fn new(link: Scope<C>) -> Self {
        Self { link }
    }
}

impl<C: Component> AsyncCallback<C::Message> for AsyncMessageCallback<C> {
    fn dispatch(&self, future: Pin<Box<dyn Future<Output = C::Message>>>) {
        self.link.send_future(future);
    }
}

impl<T> Component for Autocomplete<T>
where
    T: 'static + PartialEq + Clone + RenderHtml,
{
    type Message = Msg<T>;

    type Properties = Props<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            state: AutocompleteState::new(
                ctx.props().multi_select,
                ctx.props().onchange.clone(),
                AsyncMessageCallback::new(ctx.link().clone()),
                ctx.props().resolve_items.clone(),
            ),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnInput(value) => {
                self.state.oninput(value.as_str());
                true
            }
            Msg::OnKeydown(key) => {
                match key {
                    13 => self.state.select_current(),
                    38 => self.state.set_highlight_item(&HighlightDirection::Previous),
                    40 => self.state.set_highlight_item(&HighlightDirection::Next),
                    _ => (), // Noop
                };
                true
            }
            Msg::SetItems(items) => {
                self.state.set_items(items);
                true
            }
            Msg::SelectItem(index) => {
                self.state.select_item(index);
                true
            }
            Msg::Noop => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let input_callbacks = InputCallbacks {
            on_input: ctx.link().callback(Msg::OnInput),
            on_keydown: ctx.link().callback(|e: KeyboardEvent| {
                let code = e.which();

                match code {
                    // This is not tested in cypres because `type`'s behaviour when hitting up and
                    // down arrow was different, it didn't move the cursor. While in the browser it
                    // jumped from beginning of the test to the end While in the browser it jumped
                    // from beginning of the test to the end
                    13 | 38 | 40 => e.prevent_default(),
                    _ => (),
                };

                Msg::OnKeydown(code)
            }),
            select_item: ctx.link().callback(Msg::SelectItem),
        };
        let selected_items = if ctx.props().show_selected {
            Rc::new(self.state.selected_items())
        } else {
            Rc::new(Vec::new())
        };

        let view_context = view::Context {
            value: self.state.input(),
            callbacks: input_callbacks,
            items: Rc::new(self.state.items()),
            highlighted: self.state.highlighted_item(),
            selected_items,
            auto: ctx.props().auto,
        };

        html! {
            <ContextProvider<view::Context<T>> context={view_context}>
                {for ctx.props().children.iter() }
            </ContextProvider<view::Context<T>>>
        }
    }
}
