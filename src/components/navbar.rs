use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[component]
pub fn NavBar() -> Html {
    html!(
    <>
        <style>{include_str!("../css/navbar.css")}</style>
        <nav class="navbar">
            <div class="navbar-brand">
                <Link<Route> to={Route::Home}>{"Movie App"}</Link<Route>>
            </div>
            <div class="navbar-links">
                <Link<Route> to={Route::Home} classes="nav-link">{"Home"}</Link<Route>>
                <Link<Route> to={Route::Favorites} classes="nav-link">{"Favorites"}</Link<Route>>
            </div>
        </nav>
    </>
    )
}
