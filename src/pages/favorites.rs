use yew::prelude::*;

use crate::{
    components::movie_card::MovieCard,
    contexts::movie_context::{MovieContext, use_movie_context},
};

#[component]
pub fn Favorites() -> Html {
    let MovieContext { favorites, .. } = use_movie_context().expect("Movie Context not found");

    if !favorites.is_empty() {
        return html!(
            <div class="favorites">
                <h2>{"Your Favorites"}</h2>
                <div class="movies-grid">
                {
                    favorites
                        .iter()
                        .map(|movie| html!(<MovieCard movie={movie.clone()} key={movie.id}/>))
                        .collect::<Html>()
                }
                </div>
            </div>
        );
    }

    html!(
    <div>
        <style>{include_str!("../css/favorites.css")}</style>
        <h2>{"No Favorite Movies Yet"}</h2>
        <p>{"Start adding movies to your favorites and they will appear here!"}</p>
    </div>
    )
}
