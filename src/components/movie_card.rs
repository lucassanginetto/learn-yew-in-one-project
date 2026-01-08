use yew::prelude::*;

use crate::{
    contexts::movie_context::{MovieContext, use_movie_context},
    services::Movie,
};

#[derive(Properties, PartialEq)]
pub struct MovieCardProps {
    pub movie: Movie,
}

#[component]
pub fn MovieCard(MovieCardProps { movie }: &MovieCardProps) -> Html {
    let movie_context = use_movie_context().expect("Movie Context not found");
    let favorite = movie_context.is_favorite(movie.id);

    let MovieContext {
        add_to_favorites,
        remove_from_favorites,
        ..
    } = movie_context;

    let on_favorite_click = {
        let movie = movie.clone();
        let favorite = favorite.clone();
        let remove_from_favorites = remove_from_favorites.clone();
        let add_to_favorites = add_to_favorites.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            if favorite {
                remove_from_favorites.emit(movie.id)
            } else {
                add_to_favorites.emit(movie.clone())
            }
        })
    };

    html!(
        <div class="movie-card">
            <div class="movie-poster">
                <img src={format!("https://image.tmdb.org/t/p/w500{}", movie.poster_path)} alt={movie.title.clone()}/>
                <div class="movie-overlay">
                    <button class={format!("favorite-btn {}", if favorite {"active"} else {""})} onclick={on_favorite_click}>
                        {"♥"}
                    </button>
                </div>
            </div>
            <div class="movie-info">
                <h3>{movie.title.clone()}</h3>
                <p>{movie.release_date.split("-").next().unwrap()}</p>
            </div>
        </div>
    )
}
