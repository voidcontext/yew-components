use yew::Callback;

use crate::{Dispatcher, ItemResolver, Msg};

pub enum HighlightDirection {
    Previous,
    Next,
}

pub struct AutocompleteState<T> {
    // State
    input: String,
    items: Vec<T>,
    highlighted_item: Option<usize>,
    selected_items: Vec<T>,
    onselect: Callback<Vec<T>>,

    // Config
    multi_select: bool,
}

impl<T> Default for AutocompleteState<T> {
    fn default() -> Self {
        Self {
            input: String::default(),
            items: Vec::default(),
            highlighted_item: None,
            selected_items: Vec::default(),
            onselect: Callback::from(|_| ()),
            multi_select: false,
        }
    }
}

impl<T> AutocompleteState<T>
where
    T: 'static + Clone + PartialEq,
{
    pub fn new(multi_select: bool, onselect: Callback<Vec<T>>) -> Self {
        Self {
            onselect,
            multi_select,
            ..Self::default()
        }
    }

    // ### Input
    pub fn input(&self) -> String {
        self.input.clone()
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

    // ### Items
    pub fn items(&self) -> Vec<T> {
        self.items.clone()
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.highlighted_item = None;
    }

    // ### Item highlight
    pub fn highlighted_item(&self) -> Option<usize> {
        self.highlighted_item
    }
    pub fn set_highlight_item(&mut self, direction: &HighlightDirection) {
        match direction {
            HighlightDirection::Next => {
                let new_index = self.highlighted_item.map_or(0, |old| old + 1);

                if new_index < self.items.len() {
                    self.highlighted_item = Some(new_index);
                }
            }
            HighlightDirection::Previous => {
                if let Some(old) = self.highlighted_item {
                    if old != 0 {
                        self.highlighted_item = Some(old - 1);
                    }
                }
            }
        }
    }

    // # Selected items
    pub fn selected_items(&self) -> Vec<T> {
        self.selected_items.clone()
    }

    pub fn select_current(&mut self) {
        if let Some(selected) = self.highlighted_item {
            if self.multi_select {
                if !self
                    .selected_items
                    .iter()
                    .any(|item| *item == self.items[selected])
                {
                    self.selected_items.push(self.items[selected].clone());
                }
            } else {
                self.selected_items = vec![self.items[selected].clone()];
            }

            self.onselect.emit(self.selected_items.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::{
        marker::PhantomData,
        pin::Pin,
        sync::{Arc, Mutex},
    };

    use crate::{Dispatcher, ItemResolverResult, Msg};

    use super::{AutocompleteState, HighlightDirection};

    use futures::{channel::oneshot::Sender, Future};
    use wasm_bindgen_futures::spawn_local;
    use wasm_bindgen_test::wasm_bindgen_test;

    use yew::Callback;
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

    fn noop_callback<T>() -> Callback<T> {
        Callback::from(|_| ())
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

    // --- set_items

    #[wasm_bindgen_test]
    fn test_set_items_sets_the_items() {
        let mut state = AutocompleteState::default();

        state.set_items(vec!["one"]);

        assert_eq!(state.items, vec!["one"]);

        state.set_items(vec!["one", "two"]);

        assert_eq!(state.items, vec!["one", "two"]);
    }

    #[wasm_bindgen_test]
    fn test_set_items_resets_the_highlighted_item() {
        let mut state = AutocompleteState::default();

        state.set_items(vec!["one"]);
        state.set_highlight_item(&HighlightDirection::Next);
        state.set_items(vec!["one", "two"]);

        assert_eq!(state.highlighted_item(), None);
    }
    // --- highlight items

    #[wasm_bindgen_test]
    fn test_there_is_not_any_highlighted_items_by_default() {
        let state = AutocompleteState::<&str>::default();
        assert_eq!(state.highlighted_item(), None);
    }

    #[wasm_bindgen_test]
    fn test_highlight_item_next_should_highlight_first_when_no_highlighted_and_there_are_items() {
        let mut state = AutocompleteState::<&str>::default();

        state.set_items(vec!["foo", "bar", "baz"]);

        state.set_highlight_item(&HighlightDirection::Next);

        assert_eq!(state.highlighted_item(), Some(0));
    }

    #[wasm_bindgen_test]
    fn test_highlight_item_next_should_highlight_nothing_when_there_are_not_any_items() {
        let mut state = AutocompleteState::<&str>::default();

        state.set_highlight_item(&HighlightDirection::Next);

        assert_eq!(state.highlighted_item(), None);
    }

    #[wasm_bindgen_test]
    fn test_highlight_item_next_should_highlight_next() {
        let mut state = AutocompleteState::<&str>::default();

        state.set_items(vec!["foo", "bar", "baz"]);

        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);

        assert_eq!(state.highlighted_item(), Some(1));
    }

    #[wasm_bindgen_test]
    fn test_highlight_item_next_should_stop_at_the_end() {
        let mut state = AutocompleteState::<&str>::default();

        state.set_items(vec!["foo"]);

        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);

        assert_eq!(state.highlighted_item(), Some(0));
    }

    #[wasm_bindgen_test]
    fn test_highlight_item_previous_should_highlight_previous() {
        let mut state = AutocompleteState::<&str>::default();

        state.set_items(vec!["foo", "bar", "baz"]);

        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Previous);

        assert_eq!(state.highlighted_item(), Some(0));
    }

    #[wasm_bindgen_test]
    fn test_highlight_item_previous_should_stop_at_first() {
        let mut state = AutocompleteState::<&str>::default();

        state.set_items(vec!["foo", "bar", "baz"]);

        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Previous);
        state.set_highlight_item(&HighlightDirection::Previous);

        assert_eq!(state.highlighted_item(), Some(0));
    }

    #[wasm_bindgen_test]
    fn test_highlight_item_previous_should_highlight_nothing_when_there_are_not_any_items() {
        let mut state = AutocompleteState::<&str>::default();

        state.set_highlight_item(&HighlightDirection::Previous);

        assert_eq!(state.highlighted_item(), None);
    }

    // --- select items
    #[wasm_bindgen_test]
    fn test_selected_items_is_empty_by_default() {
        let state = AutocompleteState::<&str>::default();
        assert_eq!(state.selected_items(), Vec::<&str>::new());
    }

    #[wasm_bindgen_test]
    fn test_select_current_should_select_currently_highlighted_item() {
        let mut state = AutocompleteState::<&str>::new(false, noop_callback());

        state.set_items(vec!["foo", "bar", "baz"]);

        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);

        state.select_current();

        assert_eq!(state.selected_items(), vec!["bar"]);
    }

    #[wasm_bindgen_test]
    fn test_select_current_should_replace_the_selected_item_when_not_multi() {
        let mut state = AutocompleteState::<&str>::new(false, noop_callback());

        state.set_items(vec!["foo", "bar", "baz"]);

        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        assert_eq!(state.selected_items(), vec!["bar"]);
    }
    #[wasm_bindgen_test]
    fn test_select_current_should_select_multiple_items_if_configured() {
        let mut state = AutocompleteState::<&str>::new(true, noop_callback());

        state.set_items(vec!["foo", "bar", "baz"]);

        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        assert_eq!(state.selected_items(), vec!["foo", "bar"]);
    }

    #[wasm_bindgen_test]
    fn test_select_current_should_never_select_the_same_item_twice() {
        let mut state = AutocompleteState::<&str>::new(true, noop_callback());

        state.set_items(vec!["foo", "bar", "baz"]);

        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();
        state.select_current();

        assert_eq!(state.selected_items(), vec!["foo"]);
    }

    #[wasm_bindgen_test]
    fn test_select_current_should_emit_onselect_callback() {
        let emitted = Arc::new(Mutex::new(Vec::<Vec<String>>::new()));
        let onselect = {
            let emitted = Arc::clone(&emitted);
            Callback::from(move |strs: Vec<String>| {
                let mut guard = emitted.lock().unwrap();
                (*guard).push(strs);
            })
        };

        let mut state = AutocompleteState::<String>::new(true, onselect);

        state.set_items(vec![
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
        ]);

        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        assert_eq!(
            *emitted.lock().unwrap(),
            vec![
                vec!["foo".to_string()],
                vec!["foo".to_string(), "bar".to_string()]
            ]
        );
    }
}
