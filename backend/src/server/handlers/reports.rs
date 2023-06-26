use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Result,
    Json,
};
use expenser::{NewReport, Report};

pub async fn create_report<'a>(
    State(state): State<AppState>,
    Json(payload): Json<NewReport>,
) -> Result<Json<Report>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match payload.insert(database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn get_report(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Report>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match Report::get_by_id(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

pub async fn update_report<'a>(
    Path(path): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<NewReport>,
) -> Result<Json<Report>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match Report::update(
        path,
        payload.owner_id,
        payload.title,
        payload.description,
        database_connection,
    ) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn delete_report(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Report>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match Report::delete(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}
