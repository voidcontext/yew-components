use crate::{Dispatcher, ItemResolver, Msg};

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

impl<T> AutocompleteState<T>
where
    T: 'static + Clone,
{
    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn items(&self) -> Vec<T> {
        self.items.clone()
    }

    pub fn oninput<D: Dispatcher<Msg<T>>>(
        &mut self,
        value: &str,
        dispatcher: D,
        item_resolver: ItemResolver<T>,
    ) {
        self.input = value.to_string();

        let string = self.input.clone();

        // TODO: make the min length configurable
        if string.len() > 2 {
            dispatcher.dispatch(Box::pin(async move {
                let items = (item_resolver.fun)(string).await;

                Msg::SetItems(items.unwrap())
            }));
        } else {
            self.items = vec![];
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::{marker::PhantomData, pin::Pin};

    use crate::{Dispatcher, ItemResolverResult, Msg};

    use super::AutocompleteState;

    use futures::{channel::oneshot::Sender, Future};
    use wasm_bindgen_futures::spawn_local;
    use wasm_bindgen_test::wasm_bindgen_test;

    use yew_commons::FnProp;

    type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

    struct DispatcherMock<T> {
        f: Box<dyn FnOnce(PinnedFuture<T>)>,
        _t: PhantomData<T>,
    }

    impl<T: 'static + std::fmt::Debug> DispatcherMock<T> {
        fn noop() -> Self {
            Self {
                f: Box::new(|_f| {}),
                _t: PhantomData,
            }
        }

        fn forward(tx: Sender<T>) -> Self {
            Self {
                f: Box::new(|f| {
                    spawn_local(async move {
                        let msg = f.await;
                        tx.send(msg).unwrap();
                    });
                }),
                _t: PhantomData,
            }
        }

        fn never_called() -> Self {
            Self {
                f: Box::new(|_f| {
                    panic!("shouldn't be called");
                }),
                _t: PhantomData,
            }
        }
    }

    impl<T> Dispatcher<T> for DispatcherMock<T> {
        fn dispatch(self, future: Pin<Box<dyn futures::Future<Output = T>>>) {
            (self.f)(future);
        }
    }

    // --- oninput

    #[wasm_bindgen_test]
    fn test_oninput_sets_input_value() {
        let mut state = AutocompleteState::default();

        state.oninput(
            "this is a text",
            DispatcherMock::noop(),
            FnProp::from(|_s: String| -> ItemResolverResult<String> {
                Box::pin(futures::future::ok(vec![]))
            }),
        );

        assert_eq!(state.input, "this is a text");
    }

    #[wasm_bindgen_test]
    async fn test_oninput_should_resolve_autocomplete_items() {
        let mut state = AutocompleteState::default();

        let (tx, rx) = futures::channel::oneshot::channel::<Msg<String>>();

        state.oninput(
            "this is a text",
            DispatcherMock::forward(tx),
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
            DispatcherMock::never_called(),
            FnProp::from(|_s: String| -> ItemResolverResult<String> {
                panic!("Shouldn't be called");
            }),
        );

        assert_eq!(state.input(), "th".to_string());
    }

    #[wasm_bindgen_test]
    fn test_oninput_should_clear_items_when_input_is_short() {
        let mut state = AutocompleteState::default();
        state.set_items(vec!["one".to_string(), "two".to_string()]);

        state.oninput(
            "th",
            DispatcherMock::never_called(),
            FnProp::from(|_s: String| -> ItemResolverResult<String> {
                panic!("Shouldn't be called")
            }),
        );

        assert_eq!(state.items(), Vec::<String>::new());
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
