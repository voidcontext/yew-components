use std::rc::Rc;
use yew::Properties;

/// Wraps functions to be passed around as Yew property
///
/// This wrapper is very similar to [yew::Callback], but for arbitrary functions.
#[derive(Properties)]
pub struct FnProp<In, Out> {
    pub fun: Rc<dyn Fn(In) -> Out>,
}

impl<In, Out, F> From<F> for FnProp<In, Out>
where
    F: 'static + Fn(In) -> Out,
{
    fn from(fun: F) -> Self {
        Self { fun: Rc::new(fun) }
    }
}

#[allow(clippy::vtable_address_comparisons)]
impl<In, Out> PartialEq for FnProp<In, Out> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.fun, &other.fun)
    }
}

/// The Clone implementation "clones" the internal Rc pointer by incrementing the ref count
impl<In, Out> Clone for FnProp<In, Out> {
    fn clone(&self) -> Self {
        Self {
            fun: Rc::clone(&self.fun),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::FnProp;

    #[wasm_bindgen_test]
    fn clone_increments_counter() {
        let fn_prop = FnProp::from(|_: String| "it works!");

        assert_eq!(Rc::strong_count(&fn_prop.fun), 1);

        let another_one = fn_prop.clone();

        assert_eq!(Rc::strong_count(&fn_prop.fun), 2);
        assert_eq!(Rc::strong_count(&another_one.fun), 2);
    }

    #[wasm_bindgen_test]
    fn eq_itself() {
        let fn_prop = FnProp::from(|_: String| "it works!");

        assert!(fn_prop == fn_prop)
    }

    #[wasm_bindgen_test]
    fn eq_cloned() {
        let fn_prop = FnProp::from(|_: String| "it works!");
        let cloned = fn_prop.clone();

        assert!(fn_prop == cloned)
    }

    #[wasm_bindgen_test]
    fn not_eq() {
        let fn_prop = FnProp::from(|_: String| "it works!");
        let other = FnProp::from(|_: String| "it works!");

        assert!(fn_prop != other)
    }

    #[wasm_bindgen_test]
    fn not_eq_if_re_wrapped_in_rc() {
        let fun = |_: String| "it works!";
        let fn_prop = FnProp::from(fun);
        let other = FnProp::from(fun);

        assert!(fn_prop != other)
    }
}
