use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Result,
    Json,
};
use expenser::{NewReportAccess, ReportAccess};

#[axum::debug_handler]
pub async fn create_access(
    State(state): State<AppState>,
    Json(payload): Json<NewReportAccess>,
) -> Result<Json<ReportAccess>, StatusCode> {
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
pub async fn get_access_by_report(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ReportAccess>>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportAccess::get_by_report(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn get_access(
    Path(path): Path<(i64, i64)>,
    State(state): State<AppState>,
) -> Result<Json<ReportAccess>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportAccess::get_by_path(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn update_access(
    Path(path): Path<(i64, i64)>,
    State(state): State<AppState>,
    Json(payload): Json<NewReportAccess>,
) -> Result<Json<ReportAccess>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportAccess::replace(path, &payload, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn delete_access(
    Path(path): Path<(i64, i64)>,
    State(state): State<AppState>,
) -> Result<Json<ReportAccess>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportAccess::delete(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn clear_access(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<(), StatusCode> {
    let database_connection = &mut state.get_conn()?;

    if let Err(e) = ReportAccess::clear_by_report(path, database_connection) {
        log::error!("{e}");
        Err(StatusCode::BAD_GATEWAY)
    } else {
        Ok(())
    }
}
