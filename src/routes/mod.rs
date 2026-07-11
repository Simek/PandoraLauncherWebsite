use yew::prelude::*;
use yew_router::prelude::*;

mod home;
mod discord;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/discord")]
    Discord,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<home::Home />},
        AppRoute::Discord => html! { <discord::Discord />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}
