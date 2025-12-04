use axum::{
    Router,
    routing::{delete, get, post, put},
};
use registry::AppRegistry;

use crate::handler::manga::{delete_manga, list_mangas, register_manga, update_manga};

pub fn build_manga_routers() -> Router<AppRegistry> {
    let router = Router::new()
        .route("/", post(register_manga))
        .route("/", get(list_mangas))
        .route("/:manga_id", put(update_manga))
        .route("/:manga_id", delete(delete_manga));

    Router::new().nest("/mangas", router)
}
