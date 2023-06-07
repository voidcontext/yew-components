# yew-autocomplete [![build status badge](https://woodpecker.ci.vdx.hu/api/badges/voidcontext/yew-components/status.svg)](https://woodpecker.ci.vdx.hu/voidcontext/yew-components)

A highly configurable auto completion component for [yew.rs](https://yew.rs).

A minimal example that showcases the main building blocks:

```rust
use yew::prelude::*;
use yew_autocomplete::{view::Bulma, Autocomplete, ItemResolver, ItemResolverResult};
use yew_commons::FnProp;

let onchange = |_: Vec<String>| ();
let resolve_items: ItemResolver<String> =
    FnProp::from(|_: String| -> ItemResolverResult<String>  {
        Box::pin(async { Ok(Vec::<String>::new()) })
    });

html! {
    <Autocomplete<String>
        {resolve_items}
        {onchange}
        auto = false
    >
        <Bulma<String> />
    </Autocomplete<String>>
};
```
