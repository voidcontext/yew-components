use yew::{html, Html};

pub trait RenderHtml {
    fn render(&self) -> Html;
}

impl RenderHtml for String {
    fn render(&self) -> Html {
        html! {(*self).clone()}
    }
}
