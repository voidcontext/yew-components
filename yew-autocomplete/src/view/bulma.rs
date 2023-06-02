use yew::prelude::*;
use yew::Html;

use crate::make_callback;

use super::RenderHtml;

pub struct Bulma<T: 'static + Clone + PartialEq> {
    value: String,
    view_context: super::Context<T>,
    _context_listener: ContextHandle<super::Context<T>>,
}

pub enum Msg<T: 'static + Clone + PartialEq> {
    OnInput(String),
    Search,
    ViewContextUpdated(super::Context<T>),
}

fn render_if(when: bool, html: Html) -> Html {
    if when {
        html
    } else {
        Html::default()
    }
}

impl<T: 'static + Clone + PartialEq + RenderHtml> Component for Bulma<T> {
    type Message = Msg<T>;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (view_context, context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::ViewContextUpdated))
            .expect("No View Context Provided");

        Self {
            value: view_context.value.clone(),
            view_context,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ViewContextUpdated(ctx) => {
                self.view_context = ctx;
                true
            }
            Msg::OnInput(value) => {
                self.value = value;

                if self.view_context.config.auto {
                    self.view_context
                        .callbacks
                        .on_input
                        .emit(self.value.clone());
                    false
                } else {
                    true
                }
            }
            Msg::Search => {
                self.view_context
                    .callbacks
                    .on_input
                    .emit(self.value.clone());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let items_lis = self
            .view_context
            .items
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let mut classes = vec!["dropdown-item autocomplete-item"];

                if self.view_context.highlighted.iter().any(|h| *h == index) {
                    classes.push("highlighted");
                    classes.push("is-active");
                }

                html! { <a class={classes!(classes)}>{value.render()}</a>}
            })
            .collect::<Html>();
        let selected_lis = self
            .view_context
            .selected_items
            .iter()
            .map(|value| {
                html! { <li class={classes!("autocomplete-item", "selected")}>{value.render()}</li>}
            })
            .collect::<Html>();

        let oninput = make_callback(ctx.link(), Msg::OnInput);
        let onsearch = ctx.link().callback(|_| Msg::Search);

        html! {
            <div>
                {
                    render_if(!self.view_context.selected_items.is_empty(), html!{
                        <ul class="selected-items">
                            { selected_lis }
                        </ul>
                    })
                }
                <div class="field">
                    <div class="field has-addons" style="margin-bottom: 0">
                        <div class="control is-expanded">
                            <input
                                class="input"
                                type="text"
                                value={self.value.clone()}
                                {oninput}
                                onkeydown={self.view_context.callbacks.on_keydown.clone()}
                            />
                        </div>
                    </div>
                    {
                        render_if(!self.view_context.items.is_empty(), html!{
                            <div class="dropdown is-active autocomplete-items">
                                <div class="dropdown-menu">
                                    <div class="dropdown-content">
                                        { items_lis }
                                    </div>
                                </div>
                            </div>
                        })
                    }
                </div>
                {
                    render_if(
                        !self.view_context.config.auto,
                        html! {
                            <input type="button" value="Search" onclick={onsearch}/>
                        }
                    )
                }
            </div>
        }
    }
}
