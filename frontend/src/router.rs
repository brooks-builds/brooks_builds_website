use super::views::home::Home;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Home,
}

pub fn switch(routes: &Routes) -> Html {
    match routes {
        Routes::Home => html! { <Home /> },
    }
}
