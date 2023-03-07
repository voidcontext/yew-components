use std::{future::Future, pin::Pin};

use crate::{ItemResolver, Msg};

pub struct AutocompleteState<T> {
    input: String,
    items: Vec<T>,
}

impl<T> Default for AutocompleteState<T> {
    fn default() -> Self {
        Self {
            input: String::default(),
            items: Vec::default(),
        }
    }
}

impl<T: Clone + 'static> AutocompleteState<T> {
    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn items(&self) -> Vec<T> {
        self.items.clone()
    }

    pub fn oninput(
        &mut self,
        value: &str,
        dispatcher: impl FnOnce(Pin<Box<dyn Future<Output = Msg<T>>>>),
        ir: ItemResolver<T>,
    ) {
        self.input = value.to_string();

        let string = self.input.clone();

        // TODO: make the min length configurable
        if string.len() > 2 {
            dispatcher(Box::pin(async move {
                let items = (ir.fun)(string).await;

                Msg::SetItems(items.unwrap())
            }));
        } else {
            dispatcher(Box::pin(async move { Msg::SetItems(vec![]) }));
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::{ItemResolverResult, Msg};

    use super::AutocompleteState;

    use wasm_bindgen_futures::spawn_local;
    use wasm_bindgen_test::wasm_bindgen_test;
    use yew_commons::FnProp;

    #[wasm_bindgen_test]
    fn test_oninput_sets_input_value() {
        let mut state = AutocompleteState::default();

        state.oninput(
            "this is a text",
            |_f| (),
            FnProp::from(|_s: String| -> ItemResolverResult<String> {
                Box::pin(futures::future::ok(vec![]))
            }),
        );

        assert_eq!(state.input, "this is a text");
    }

    // --- oninput

    #[wasm_bindgen_test]
    async fn test_oninput_should_resolve_autocomplete_items() {
        let mut state = AutocompleteState::default();

        let (tx, rx) = futures::channel::oneshot::channel::<Msg<String>>();

        state.oninput(
            "this is a text",
            |f| {
                spawn_local(async move {
                    let msg = f.await;
                    tx.send(msg).unwrap();
                });
            },
            FnProp::from(|_s: String| -> ItemResolverResult<String> {
                Box::pin(futures::future::ok(vec!["result".to_string()]))
            }),
        );

        let sent = rx.await.unwrap();
        assert_eq!(sent, Msg::SetItems(vec!["result".to_string()]));
    }

    #[wasm_bindgen_test]
    fn test_oninput_should_not_resolve_autocomplete_items_when_input_is_short() {
        let mut state = AutocompleteState::default();

        state.oninput(
            "th",
            |_f| {},
            FnProp::from(|_s: String| -> ItemResolverResult<String> {
                panic!("Shouldn't be called");
            }),
        );

        assert_eq!(state.input(), "th".to_string());
    }

    #[wasm_bindgen_test]
    async fn test_oninput_should_clear_items_when_input_is_short() {
        let mut state = AutocompleteState::default();

        let (tx, rx) = futures::channel::oneshot::channel::<Msg<String>>();

        state.oninput(
            "th",
            |f| {
                spawn_local(async move {
                    let msg = f.await;
                    tx.send(msg).unwrap();
                });
            },
            FnProp::from(|_s: String| -> ItemResolverResult<String> {
                panic!("Shouldn't be called")
            }),
        );

        let sent = rx.await.unwrap();
        assert_eq!(sent, Msg::SetItems(vec![]));
    }

    // --- oninput

    #[wasm_bindgen_test]
    fn test_set_items_sets_the_items() {
        let mut state = AutocompleteState::default();

        state.set_items(vec!["one"]);

        assert_eq!(state.items, vec!["one"]);

        state.set_items(vec!["one", "two"]);

        assert_eq!(state.items, vec!["one", "two"]);
    }
}
