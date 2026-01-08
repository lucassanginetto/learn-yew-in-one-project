use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

use crate::services::Movie;

#[derive(Clone, PartialEq)]
pub struct MovieContext {
    pub favorites: UseStateHandle<Vec<Movie>>,
    pub add_to_favorites: Callback<Movie>,
    pub remove_from_favorites: Callback<u32>,
}

impl MovieContext {
    pub fn is_favorite(&self, movie_id: u32) -> bool {
        self.favorites.iter().any(|movie| movie.id == movie_id)
    }
}

pub fn use_movie_context() -> impl Hook<Output = Option<MovieContext>> {
    use_context::<MovieContext>()
}

#[derive(Properties, PartialEq)]
pub struct MovieProviderProps {
    pub children: Html,
}

#[component]
pub fn MovieProvider(MovieProviderProps { children }: &MovieProviderProps) -> Html {
    let favorites = use_state(|| Vec::<Movie>::new());

    {
        let favorites = favorites.clone();
        use_effect_with((), move |_| {
            if let Ok(stored_favs) = LocalStorage::get("favorites")
                && *favorites != stored_favs
            {
                favorites.set(stored_favs);
            }
        });
    }

    {
        let favorites = favorites.clone();
        use_effect_with(favorites.clone(), move |_| {
            let _ = LocalStorage::set("favorites", &*favorites);
        });
    }

    let add_to_favorites = {
        let favorites = favorites.clone();
        Callback::from(move |movie| {
            let mut favs = Vec::from((*favorites).clone());
            favs.push(movie);
            favorites.set(favs);
        })
    };

    let remove_from_favorites = {
        let favorites = favorites.clone();
        Callback::from(move |movie_id: u32| {
            let favs = Vec::from((*favorites).clone());
            favorites.set(
                favs.into_iter()
                    .filter(|movie| movie.id != movie_id)
                    .collect(),
            );
        })
    };

    html!(
        <ContextProvider<MovieContext> context={MovieContext{favorites, add_to_favorites, remove_from_favorites}}>
            {children}
        </ContextProvider<MovieContext>>
    )
}
