use std::{cell::RefCell, rc::Rc};

use yew::Callback;

use crate::{AsyncCallback, ItemResolver, Msg};

pub enum HighlightDirection {
    Previous,
    Next,
}

pub(crate) struct AutocompleteState<T, D: AsyncCallback<Msg>> {
    // State
    input: String,
    items: Rc<RefCell<Vec<T>>>,
    highlighted_item: Rc<RefCell<Option<usize>>>,
    selected_items: Vec<T>,
    onselect: Callback<Vec<T>>,

    dispatcher: D,
    item_resolver: ItemResolver<T>,

    auto: bool,
    multi_select: bool,
}

impl<T, D: AsyncCallback<Msg>> AutocompleteState<T, D>
where
    T: 'static + Clone + PartialEq,
{
    pub fn new(
        auto: bool,
        multi_select: bool,
        onselect: Callback<Vec<T>>,
        dispatcher: D,
        item_resolver: ItemResolver<T>,
    ) -> Self {
        Self {
            input: String::default(),
            items: Rc::new(RefCell::new(Vec::new())),
            highlighted_item: Rc::new(RefCell::new(None)),
            selected_items: Vec::default(),
            dispatcher,
            item_resolver,
            onselect,
            auto,
            multi_select,
        }
    }

    // ### Input
    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn oninput(&mut self, value: &str) {
        self.input = value.to_string();

        // TODO: make the min length configurable
        if self.input.len() > 2 && self.auto {
            self.resolve();
        } else {
            let mut guard = self.items.borrow_mut();
            *guard = Vec::new();

            // self.highlighted_item = None;
        }
    }

    // TODO: refactor
    //       just require a callback, then spawn a local thread internally to evaluate the future and call the callback
    // by using the dispatcher's ability to run evaluate futures:
    // - resolves the items
    // - stores the item in the state
    pub fn resolve(&self) {
        let string = self.input.clone();
        let item_resolver = self.item_resolver.clone();

        let rc_items = Rc::clone(&self.items);
        let rc_highlighted = Rc::clone(&self.highlighted_item);
        self.dispatcher.dispatch(Box::pin(async move {
            let items = (item_resolver.fun)(string).await.unwrap();

            *rc_items.borrow_mut() = items;
            *rc_highlighted.borrow_mut() = None;

            Msg::Noop(true)
        }));
    }

    // ### Items
    pub fn items(&self) -> Vec<T> {
        (*self.items).borrow().clone()
    }

    // ### Item highlight
    pub fn highlighted_item(&self) -> Option<usize> {
        *(*self.highlighted_item).borrow()
    }

    pub fn set_highlight_item(&mut self, direction: &HighlightDirection) {
        match direction {
            HighlightDirection::Next => {
                let new_index = (*self.highlighted_item).borrow().map_or(0, |old| old + 1);

                if new_index < (*self.items).borrow().len() {
                    *self.highlighted_item.borrow_mut() = Some(new_index);
                }
            }
            HighlightDirection::Previous => {
                let old = *(*self.highlighted_item).borrow();
                if let Some(index) = old {
                    if index != 0 {
                        *self.highlighted_item.borrow_mut() = Some(index - 1);
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
        let selected = *(*self.highlighted_item).borrow();
        if let Some(index) = selected {
            self.select_item(index);
        }
    }

    pub fn select_item(&mut self, index: usize) {
        let mut items = self.items.borrow_mut();

        if self.multi_select {
            if !self.selected_items.iter().any(|item| *item == items[index]) {
                self.selected_items.push(items[index].clone());
            }
        } else {
            self.selected_items = vec![items[index].clone()];
        }

        self.input = String::new();
        *items = Vec::new();
        self.onselect.emit(self.selected_items.clone());
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

    use crate::{AsyncCallback, ItemResolverResult, Msg};

    use super::{AutocompleteState, HighlightDirection};

    use futures::{channel::mpsc::Receiver, Future, StreamExt};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::{spawn_local, JsFuture};
    use wasm_bindgen_test::wasm_bindgen_test;

    use yew::Callback;
    use yew_commons::FnProp;

    type PinnedFuture<T> = Pin<Box<dyn Future<Output = T>>>;

    struct DispatcherMock<T> {
        f: Box<dyn Fn(PinnedFuture<T>)>,
        _t: PhantomData<T>,
    }

    impl<T: 'static + std::fmt::Debug> DispatcherMock<T> {
        fn noop() -> Self {
            Self {
                f: Box::new(move |f| {
                    spawn_local(async move {
                        f.await;
                    });
                }),
                _t: PhantomData,
            }
        }

        fn forward() -> (Self, Receiver<T>) {
            let (tx, rx) = futures::channel::mpsc::channel::<T>(10);

            let state = Self {
                f: Box::new(move |f| {
                    let mut tx = tx.clone();
                    spawn_local(async move {
                        let msg = f.await;
                        tx.try_send(msg).unwrap();
                    });
                }),
                _t: PhantomData,
            };

            (state, rx)
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

    impl<T> AsyncCallback<T> for DispatcherMock<T> {
        fn dispatch(&self, future: Pin<Box<dyn futures::Future<Output = T>>>) {
            (self.f)(future);
        }
    }

    fn not_resolved_default_state<T: std::fmt::Debug + Clone + PartialEq + 'static>(
        multi: bool,
    ) -> AutocompleteState<T, DispatcherMock<Msg>> {
        AutocompleteState::new(
            true,
            multi,
            noop_callback(),
            DispatcherMock::never_called(),
            FnProp::from(|_s: String| -> ItemResolverResult<T> {
                panic!("Shouldn't be called");
            }),
        )
    }

    fn default_state_with_static_results<T: std::fmt::Debug + Clone + PartialEq + 'static>(
        multi: bool,
        results: Vec<T>,
    ) -> AutocompleteState<T, DispatcherMock<Msg>> {
        AutocompleteState::new(
            true,
            multi,
            noop_callback(),
            DispatcherMock::noop(),
            FnProp::from(move |_s: String| -> ItemResolverResult<T> {
                let results = results.clone();
                Box::pin(async { Ok(results) })
            }),
        )
    }

    async fn tick() {
        let promise = js_sys::Promise::resolve(&JsValue::from(0));

        JsFuture::from(promise).await.unwrap();
    }

    // --- oninput

    #[wasm_bindgen_test]
    async fn test_oninput_sets_input_value() {
        let mut state = default_state_with_static_results::<String>(false, Vec::new());

        state.oninput("this is a text");
        tick().await;

        assert_eq!(state.input(), "this is a text");
    }

    #[wasm_bindgen_test]
    async fn test_oninput_should_resolve_autocomplete_items() {
        let (dispatcher, rx) = DispatcherMock::forward();

        let (resolver_tx, resolver_rx) = futures::channel::mpsc::channel::<String>(10);

        let mut state = AutocompleteState::new(
            true,
            false,
            noop_callback(),
            dispatcher,
            FnProp::from(move |s: String| -> ItemResolverResult<String> {
                let mut resolver_tx = resolver_tx.clone();
                Box::pin(async move {
                    resolver_tx.try_send(s).unwrap();
                    Ok(vec!["result".to_string()])
                })
            }),
        );

        state.oninput("this is a text");
        tick().await;

        // dispatcher has been called
        let (sent, _) = rx.into_future().await;
        assert_eq!(sent.unwrap(), Msg::Noop(true));

        // items have been resolved
        let (sent, _) = resolver_rx.into_future().await;
        assert_eq!(sent.unwrap(), "this is a text".to_string());
    }

    #[wasm_bindgen_test]
    async fn test_oninput_should_not_resolve_autocomplete_items_when_auto_false() {
        let mut state = AutocompleteState::new(
            false,
            false,
            noop_callback(),
            DispatcherMock::never_called(),
            FnProp::from(|_s: String| -> ItemResolverResult<String> {
                panic!("Shouldn't be called")
            }),
        );

        state.oninput("this is a text");
        tick().await;

        assert_eq!(state.input(), "this is a text");
    }

    #[wasm_bindgen_test]
    async fn test_oninput_should_not_resolve_autocomplete_items_when_input_is_short() {
        let mut state = not_resolved_default_state::<&str>(false);

        state.oninput("th");
        tick().await;

        assert_eq!(state.input(), "th".to_string());
    }

    #[wasm_bindgen_test]
    async fn test_oninput_should_clear_items_when_input_is_short() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["one", "two"]);

        state.oninput("the");
        tick().await;

        assert_eq!(state.items().len(), 2);

        state.oninput("th");
        tick().await;

        assert_eq!(state.items().len(), 0);
    }

    #[wasm_bindgen_test]
    async fn test_set_items_resets_the_highlighted_item() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["one", "two"]);

        state.oninput("the");
        tick().await;

        state.set_highlight_item(&HighlightDirection::Next);
        assert_eq!(state.highlighted_item(), Some(0));

        state.oninput("the ");
        tick().await;

        assert_eq!(state.highlighted_item(), None);
    }

    // TODO write test to test if the dispatcher is called

    // --- resolve

    #[wasm_bindgen_test]
    async fn test_resolve_should_resolve_autocomplete_items() {
        let (dispatcher, mut rx) = DispatcherMock::forward();

        let (resolver_tx, mut resolver_rx) = futures::channel::mpsc::channel::<String>(10);

        let mut state = AutocompleteState::new(
            false,
            false,
            noop_callback(),
            dispatcher,
            FnProp::from(move |s: String| -> ItemResolverResult<String> {
                let mut resolver_tx = resolver_tx.clone();
                Box::pin(async move {
                    resolver_tx.try_send(s).unwrap();
                    Ok(vec!["result".to_string()])
                })
            }),
        );

        state.oninput("this is a text");
        tick().await;

        // ensure oninput didn't resolve the items, TODO: this should be a separate test?
        let sent = rx.next().await;
        assert!(sent.is_none());

        let sent = resolver_rx.next().await;
        assert!(sent.is_none());

        state.resolve();
        tick().await;

        // dispatcher has been called
        let (sent, _) = rx.into_future().await;
        assert_eq!(sent.unwrap(), Msg::Noop(true));

        // items have been resolved
        let (sent, _) = resolver_rx.into_future().await;
        assert_eq!(sent.unwrap(), "this is a text".to_string());
    }

    // --- set_items

    #[wasm_bindgen_test]
    fn test_there_is_not_any_highlighted_items_by_default() {
        let state = not_resolved_default_state::<&str>(false);
        assert_eq!(state.highlighted_item(), None);
    }

    #[wasm_bindgen_test]
    async fn test_highlight_item_next_should_highlight_first_when_no_highlighted_and_there_are_items(
    ) {
        let mut state = default_state_with_static_results::<&str>(false, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;

        state.set_highlight_item(&HighlightDirection::Next);

        assert_eq!(state.highlighted_item(), Some(0));
    }

    #[wasm_bindgen_test]
    fn test_highlight_item_next_should_highlight_nothing_when_there_are_not_any_items() {
        let mut state = not_resolved_default_state::<&str>(false);

        state.set_highlight_item(&HighlightDirection::Next);

        assert_eq!(state.highlighted_item(), None);
    }

    #[wasm_bindgen_test]
    async fn test_highlight_item_next_should_highlight_next() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;

        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);

        assert_eq!(state.highlighted_item(), Some(1));
    }

    #[wasm_bindgen_test]
    async fn test_highlight_item_next_should_stop_at_the_end() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["foo"]);

        state.oninput("foo");
        tick().await;

        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);

        assert_eq!(state.highlighted_item(), Some(0));
    }

    #[wasm_bindgen_test]
    async fn test_highlight_item_previous_should_highlight_previous() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;

        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Previous);

        assert_eq!(state.highlighted_item(), Some(0));
    }

    #[wasm_bindgen_test]
    async fn test_highlight_item_previous_should_stop_at_first() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;

        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Previous);
        state.set_highlight_item(&HighlightDirection::Previous);

        assert_eq!(state.highlighted_item(), Some(0));
    }

    #[wasm_bindgen_test]
    fn test_highlight_item_previous_should_highlight_nothing_when_there_are_not_any_items() {
        let mut state = not_resolved_default_state::<&str>(false);

        state.set_highlight_item(&HighlightDirection::Previous);

        assert_eq!(state.highlighted_item(), None);
    }

    // --- select items
    #[wasm_bindgen_test]
    fn test_selected_items_is_empty_by_default() {
        let state = not_resolved_default_state::<&str>(false);
        assert_eq!(state.selected_items(), Vec::<&str>::new());
    }

    #[wasm_bindgen_test]
    async fn test_select_current_should_select_currently_highlighted_item() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;

        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);

        state.select_current();

        assert_eq!(state.selected_items(), vec!["bar"]);
    }

    #[wasm_bindgen_test]
    async fn test_select_current_should_replace_the_selected_item_when_not_multi() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;
        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        state.oninput("foo");
        tick().await;
        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        assert_eq!(state.selected_items(), vec!["bar"]);
    }
    #[wasm_bindgen_test]
    async fn test_select_current_should_select_multiple_items_if_configured() {
        let mut state = default_state_with_static_results::<&str>(true, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;
        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        state.oninput("foo");
        tick().await;
        state.set_highlight_item(&HighlightDirection::Next);
        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        assert_eq!(state.selected_items(), vec!["foo", "bar"]);
    }

    #[wasm_bindgen_test]
    async fn test_select_current_should_never_select_the_same_item_twice() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;
        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        state.oninput("foo");
        tick().await;
        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        assert_eq!(state.selected_items(), vec!["foo"]);
    }

    #[wasm_bindgen_test]
    async fn test_select_current_should_emit_onselect_callback() {
        let emitted = Arc::new(Mutex::new(Vec::<Vec<String>>::new()));
        let onselect = {
            let emitted = Arc::clone(&emitted);
            Callback::from(move |strs: Vec<String>| {
                let mut guard = emitted.lock().unwrap();
                (*guard).push(strs);
            })
        };

        let mut state = AutocompleteState::new(
            true,
            true,
            onselect,
            DispatcherMock::noop(),
            FnProp::from(|_s: String| -> ItemResolverResult<String> {
                Box::pin(async {
                    Ok(vec![
                        "foo".to_string(),
                        "bar".to_string(),
                        "baz".to_string(),
                    ])
                })
            }),
        );

        state.oninput("foo");
        tick().await;

        state.set_highlight_item(&HighlightDirection::Next);
        state.select_current();

        state.oninput("foo");
        tick().await;
        state.set_highlight_item(&HighlightDirection::Next);
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

    #[wasm_bindgen_test]
    async fn test_select_item_should_select_given_item() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;
        state.select_item(1);

        assert_eq!(state.selected_items(), vec!["bar"]);
    }

    #[wasm_bindgen_test]
    async fn test_select_item_should_reset_input_after_selecting_an_item() {
        let mut state = AutocompleteState::new(
            true,
            false,
            noop_callback(),
            DispatcherMock::noop(),
            FnProp::from(|_s: String| -> ItemResolverResult<&'static str> {
                Box::pin(async { Ok(vec!["foo", "foobar"]) })
            }),
        );

        state.oninput("foo");
        tick().await;
        assert_eq!(state.input(), "foo");

        state.select_item(1);
        assert_eq!(state.input(), "");
    }

    #[wasm_bindgen_test]
    async fn test_select_item_should_reset_items_after_selecting_an_item() {
        let mut state = AutocompleteState::new(
            true,
            false,
            noop_callback(),
            DispatcherMock::noop(),
            FnProp::from(|_s: String| -> ItemResolverResult<&'static str> {
                Box::pin(async { Ok(vec!["foo", "foobar"]) })
            }),
        );

        state.oninput("foo");
        tick().await;
        assert_eq!(state.input(), "foo");

        state.select_item(1);

        assert_eq!(state.items(), Vec::<&str>::new());
    }

    #[wasm_bindgen_test]
    async fn test_select_item_should_reset_highlighted_items_after_selecting_an_item() {
        let mut state = AutocompleteState::new(
            true,
            false,
            noop_callback(),
            DispatcherMock::noop(),
            FnProp::from(|_s: String| -> ItemResolverResult<&'static str> {
                Box::pin(async { Ok(vec!["foo", "foobar"]) })
            }),
        );

        state.oninput("foo");
        tick().await;
        assert_eq!(state.input(), "foo");

        state.select_item(1);

        assert_eq!(state.highlighted_item(), None);
    }
    #[wasm_bindgen_test]
    async fn test_select_item_should_replace_the_selected_item_when_not_multi() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;
        state.select_item(0);

        state.oninput("foo");
        tick().await;
        state.select_item(1);

        assert_eq!(state.selected_items(), vec!["bar"]);
    }
    #[wasm_bindgen_test]
    async fn test_select_item_should_select_multiple_items_if_configured() {
        let mut state = default_state_with_static_results::<&str>(true, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;
        state.select_item(0);

        state.oninput("foo");
        tick().await;
        state.select_item(1);

        assert_eq!(state.selected_items(), vec!["foo", "bar"]);
    }

    #[wasm_bindgen_test]
    async fn test_select_item_should_never_select_the_same_item_twice() {
        let mut state = default_state_with_static_results::<&str>(false, vec!["foo", "bar", "baz"]);

        state.oninput("foo");
        tick().await;
        state.select_item(0);

        state.oninput("foo");
        tick().await;
        state.select_item(0);

        assert_eq!(state.selected_items(), vec!["foo"]);
    }

    #[wasm_bindgen_test]
    async fn test_select_item_should_emit_onselect_callback() {
        let emitted = Arc::new(Mutex::new(Vec::<Vec<String>>::new()));
        let onselect = {
            let emitted = Arc::clone(&emitted);
            Callback::from(move |strs: Vec<String>| {
                let mut guard = emitted.lock().unwrap();
                (*guard).push(strs);
            })
        };

        let mut state = AutocompleteState::new(
            true,
            true,
            onselect,
            DispatcherMock::noop(),
            FnProp::from(|_s: String| -> ItemResolverResult<String> {
                Box::pin(async {
                    Ok(vec![
                        "foo".to_string(),
                        "bar".to_string(),
                        "baz".to_string(),
                    ])
                })
            }),
        );

        state.oninput("foo");
        tick().await;
        state.select_item(0);

        state.oninput("foo");
        tick().await;
        state.select_item(1);

        assert_eq!(
            *emitted.lock().unwrap(),
            vec![
                vec!["foo".to_string()],
                vec!["foo".to_string(), "bar".to_string()]
            ]
        );
    }
}
