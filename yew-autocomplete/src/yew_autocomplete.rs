use yew::prelude::*;

use crate::completion_state::CompletionState;

pub struct AutoComplete {
    state: CompletionState,
}

impl Component for AutoComplete {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
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
