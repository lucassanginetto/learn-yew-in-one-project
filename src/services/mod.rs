use std::fmt::Display;

use gloo_net::{Error as GlooNetError, http::Request};
use serde::{Deserialize, Serialize};

const API_KEY: &str = include_str!("./api_key.txt");
const BASE_URL: &str = "https://api.themoviedb.org/3";

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Movie {
    pub id: u32,
    pub poster_path: String,
    pub release_date: String,
    pub title: String,
}

#[derive(Deserialize)]
struct GetMoviePopular {
    page: u32,
    results: Vec<Movie>,
    total_pages: u32,
    total_results: u32,
}

pub struct ApiError(GlooNetError);
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API error: {}", self.0)
    }
}

pub async fn get_popular_movies() -> Result<Vec<Movie>, ApiError> {
    Request::get(&format!("{BASE_URL}/movie/popular?api_key={API_KEY}"))
        .send()
        .await
        .map_err(ApiError)?
        .json::<GetMoviePopular>()
        .await
        .map(|get_movie_popular| get_movie_popular.results)
        .map_err(ApiError)
}

pub async fn search_movies(query: &str) -> Result<Vec<Movie>, ApiError> {
    Request::get(&format!(
        "{BASE_URL}/search/movie?api_key={API_KEY}&query={}",
        uri_encode::encode_uri_component(query)
    ))
    .send()
    .await
    .map_err(ApiError)?
    .json::<GetMoviePopular>()
    .await
    .map(|get_movie_popular| get_movie_popular.results)
    .map_err(ApiError)
}
