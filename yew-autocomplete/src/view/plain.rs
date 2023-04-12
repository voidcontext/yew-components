use yew::prelude::*;
use yew::Html;

use super::RenderHtml;

pub struct Plain<T: 'static + Clone + PartialEq> {
    view_context: super::Context<T>,
    _context_listener: ContextHandle<super::Context<T>>,
}

pub enum Msg<T: 'static + Clone + PartialEq> {
    ViewContextUpdated(super::Context<T>),
}

fn render_if(when: bool, html: Html) -> Html {
    if when {
        html
    } else {
        Html::default()
    }
}

impl<T: 'static + Clone + PartialEq + RenderHtml> Component for Plain<T> {
    type Message = Msg<T>;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (view_context, context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::ViewContextUpdated))
            .expect("No View Context Provided");

        Self {
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
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let items_lis = self
            .view_context
            .items
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let mut classes = vec!["autocomplete-item"];

                if self.view_context.highlighted.iter().any(|h| *h == index) {
                    classes.push("highlighted");
                }

                html! { <li class={classes!(classes)}>{value.render()}</li>}
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

        html! {
            <div>
                {
                    render_if(!self.view_context.selected_items.is_empty(), html!{
                        <ul class="selected-items">
                            { selected_lis }
                        </ul>
                    })
                }
                <input
                    type="text"
                    value={self.view_context.value.clone()}
                    oninput={self.view_context.callbacks.on_input.clone()}
                    onkeydown={self.view_context.callbacks.on_keydown.clone()}
                />
                {
                    render_if(!self.view_context.items.is_empty(), html!{
                        <ul class="autocomplete-items">
                            { items_lis }
                        </ul>
                    })
                }
            </div>
        }
    }
}
