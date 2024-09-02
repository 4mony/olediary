use std::collections::HashMap;

use gloo::history::{AnyHistory, History, MemoryHistory};
use yew::{function_component, html, AttrValue, Html, Properties};
use yew_router::{Routable, BrowserRouter, Switch, Router};

mod views;

#[derive(Routable, Clone, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <p class="hello second">{"Home page"}</p> }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={ switch } />
        </BrowserRouter>
    }
}


#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub uri: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.uri, &props.queries)
        .unwrap();

    html! {
        <Router history={ history }>
            <Switch<Route> render={ switch } />
        </Router>
    }
}