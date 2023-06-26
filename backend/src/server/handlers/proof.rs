use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Result,
    Json,
};
use expenser::{NewReportProof, ReportProof};

pub async fn create_proof<'a>(
    State(state): State<AppState>,
    Json(payload): Json<NewReportProof>,
) -> Result<Json<ReportProof>, StatusCode> {
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
pub async fn get_proof_by_report(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ReportProof>>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportProof::get_by_report(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn get_proof(
    Path(path): Path<(i64, i64)>,
    State(state): State<AppState>,
) -> Result<Json<ReportProof>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportProof::get_by_path(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

pub async fn update_proof<'a>(
    Path(path): Path<(i64, i64)>,
    State(state): State<AppState>,
    Json(payload): Json<NewReportProof>,
) -> Result<Json<ReportProof>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportProof::replace(path, &payload, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn delete_proof(
    Path(path): Path<(i64, i64)>,
    State(state): State<AppState>,
) -> Result<Json<ReportProof>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportProof::delete(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn clear_proof(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<(), StatusCode> {
    let database_connection = &mut state.get_conn()?;

    if let Err(e) = ReportProof::clear_by_report(path, database_connection) {
        log::error!("{e}");
        Err(StatusCode::BAD_GATEWAY)
    } else {
        Ok(())
    }
}
