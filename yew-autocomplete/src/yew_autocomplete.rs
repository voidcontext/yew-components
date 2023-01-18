#[rustfmt::skip::macros(html)]
use yew::prelude::*;

use crate::completion_state::CompletionState;

pub struct AutoComplete {
    state: CompletionState,
}

impl Component for AutoComplete {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: Default::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <input type="text" />
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
