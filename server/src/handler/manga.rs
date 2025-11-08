use anyhow::Result;
use application::usecase;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use domain::manga::MangaId;
use registry::AppRegistry;

use crate::{
    handler::{MyRoughlyErrResponse, into_500},
    model::manga::{
        CreateMangaRequest, MangaResponse, UpdateMangaRequest, UpdateMangaRequestWithId,
    },
};

pub async fn register_manga(
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateMangaRequest>,
) -> Result<StatusCode, MyRoughlyErrResponse> {
    let command = req.try_into().map_err(into_500)?;

    usecase::create_manga::execute(registry.manga_repository(), command)
        .await
        .map_err(into_500)?;

    Ok(StatusCode::CREATED)
}

pub async fn update_manga(
    Path(manga_id): Path<MangaId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateMangaRequest>,
) -> Result<StatusCode, MyRoughlyErrResponse> {
    let update_manga = UpdateMangaRequestWithId::new(manga_id, req);
    let command = update_manga.try_into().map_err(into_500)?;
    usecase::update_manga::execute(registry.manga_repository(), command)
        .await
        .map_err(into_500)?;
    Ok(StatusCode::OK)
}

pub async fn delete_manga(
    Path(manga_id): Path<MangaId>,
    State(registry): State<AppRegistry>,
) -> Result<StatusCode, MyRoughlyErrResponse> {
    usecase::delete_manga::execute(registry.manga_repository(), manga_id.into())
        .await
        .map_err(into_500)?;

    Ok(StatusCode::OK)
}

pub async fn list_mangas(
    State(registry): State<AppRegistry>,
) -> Result<Json<Vec<MangaResponse>>, MyRoughlyErrResponse> {
    let mangas = registry
        .manga_repository()
        .find_all()
        .await
        .map_err(into_500)?;
    Ok(Json(mangas.into_iter().map(MangaResponse::from).collect()))
}
