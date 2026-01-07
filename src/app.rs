use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::navbar::NavBar;
use crate::contexts::movie_context::MovieProvider;
use crate::pages::{favorites::Favorites, home::Home};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/favorites")]
    Favorites,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[component]
pub fn App() -> Html {
    html!(
    <>
        <style>
            {include_str!("./css/index.css")}
            {include_str!("./css/app.css")}
        </style>
        <MovieProvider>
            <BrowserRouter>
                <NavBar/>
                <main class="main-content">
                    <Switch<Route> render={|route| match route {
                        Route::Home => html!(<Home/>),
                        Route::Favorites => html!(<Favorites/>),
                        Route::NotFound => html!(<h1>{"404"}</h1>),
                    }}/>
                </main>
            </BrowserRouter>
        </MovieProvider>
    </>
    )
}

/*
#[derive(Properties, PartialEq)]
struct TextProps {
    display: String,
}

#[component]
fn Text(TextProps { display }: &TextProps) -> Html {
    html!(
        <div>
            <p>{display}</p>
        </div>
    )
}
*/
