use super::handlers::{self, AppState};
use axum::{
    Router,
    routing::{get, post},
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/channels",
            post(handlers::create_channel).get(handlers::get_all_channels),
        )
        .route(
            "/channels/{id}",
            get(handlers::get_channel)
                .put(handlers::update_channel)
                .delete(handlers::delete_channel),
        )
        .with_state(state)
}
