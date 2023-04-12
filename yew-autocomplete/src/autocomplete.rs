use std::{future::Future, pin::Pin, rc::Rc};

use web_sys::HtmlInputElement;
use yew::{html::Scope, prelude::*};
use yew_commons::fn_prop::FnProp;

use crate::{
    autocomplete_state::{AutocompleteState, HighlightDirection},
    view::{self, InputCallbacks, RenderHtml},
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

pub struct Autocomplete<T> {
    state: AutocompleteState<T>,
}

#[derive(PartialEq, Properties)]
pub struct Props<T: PartialEq> {
    pub resolve_items: ItemResolver<T>,
    pub onchange: Callback<Vec<T>>,
    pub children: Children, // TODO: typed children?

    #[prop_or(false)]
    pub show_selected: bool,

    #[prop_or(false)]
    pub multi_select: bool,
}

#[derive(Debug, PartialEq)]
pub enum Msg<T> {
    OnInput(String),
    OnKeydown(u32),
    SetItems(Vec<T>),
}

pub trait Dispatcher<T> {
    // we could use &self, but in the tests we need to use an FnOnce to be able pass in a Sender<Msg>
    fn dispatch(self, future: Pin<Box<dyn Future<Output = T>>>);
}

struct ComponentDispatcher<C: Component> {
    link: Scope<C>,
}

impl<C: Component> ComponentDispatcher<C> {
    fn new(link: Scope<C>) -> Self {
        Self { link }
    }
}

impl<C: Component> Dispatcher<C::Message> for ComponentDispatcher<C> {
    fn dispatch(self, future: Pin<Box<dyn Future<Output = C::Message>>>) {
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
            state: AutocompleteState::new(ctx.props().multi_select, ctx.props().onchange.clone()),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnInput(value) => {
                self.state.oninput(
                    value.as_str(),
                    ComponentDispatcher::new(ctx.link().clone()),
                    ctx.props().resolve_items.clone(),
                );
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let input_callbacks = InputCallbacks {
            on_input: make_callback(link, Msg::OnInput),
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
        };

        html! {
            <ContextProvider<view::Context<T>> context={view_context}>
                {for ctx.props().children.iter() }
            </ContextProvider<view::Context<T>>>
        }
    }
}
