use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::movie_card::MovieCard;
use crate::services::{Movie, get_popular_movies, search_movies};

#[component]
pub fn Home() -> Html {
    let search_query = use_state(|| "".to_owned());
    let movies = use_state(|| Vec::<Movie>::new());
    let error = use_state(|| None);
    let loading = use_state(|| true);

    {
        let movies = movies.clone();
        let error = error.clone();
        let loading = loading.clone();
        use_effect(move || {
            let movies = movies.clone();
            let error = error.clone();
            let load_popular_movies = async move {
                match get_popular_movies().await {
                    Ok(popular_movies) => movies.set(popular_movies),
                    Err(err) => {
                        log!(err.to_string());
                        error.set(Some(err.to_string()));
                    }
                };
                loading.set(false);
            };
            wasm_bindgen_futures::spawn_local(load_popular_movies);
        });
    }

    let handle_search = {
        let search_query = search_query.clone();
        let loading = loading.clone();
        let error = error.clone();
        let movies = movies.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            if search_query.trim().is_empty() || *loading {
                return;
            }

            loading.set(true);

            {
                let movies = movies.clone();
                let search_query = search_query.clone();
                let error = error.clone();
                let loading = loading.clone();
                let load_search_movies = async move {
                    match search_movies(&*search_query).await {
                        Ok(search_results) => {
                            movies.set(search_results);
                            error.set(None);
                        }
                        Err(err) => {
                            log!(err.to_string());
                            error.set(Some("Failed to search movies...".into()));
                        }
                    }
                    loading.set(false);
                };
                wasm_bindgen_futures::spawn_local(load_search_movies);
            }
        })
    };

    html!(
    <>
        <style>
            {include_str!("../css/home.css")}
        </style>
        <div class="home">
            <form onsubmit={handle_search} class="search-form">
                <input
                    type="text"
                    placeholder="Search for movies..."
                    class="search-input"
                    value={(*search_query).clone()}
                    oninput={{
                        let search_query = search_query.clone();

                        Callback::from(move |event: InputEvent| {
                            search_query.set(
                                event
                                    .target()
                                    .unwrap()
                                    .unchecked_into::<HtmlInputElement>()
                                    .value(),
                            );
                        })
                    }}
                />
                <button type="submit" class="search-button">{"Search"}</button>
            </form>

            if let Some(err) = &*error {
                <div>{err.to_string()}</div>
            }

            if *loading {
                <div class="loading">{"Loading..."}</div>
            } else {
                <div class="movies-grid">
                {
                    movies
                        .iter()
                        .map(|movie| html!(<MovieCard movie={movie.clone()} key={movie.id}/>))
                        .collect::<Html>()
                }
                </div>
            }
        </div>
    </>
    )
}
