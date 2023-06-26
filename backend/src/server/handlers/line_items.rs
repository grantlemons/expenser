use super::types::{NewReportLineItemSerde, ReportLineItemSerde};
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Result,
    Json,
};
use expenser::{NewReportLineItem, ReportLineItem};

pub async fn create_line_item<'a>(
    State(state): State<AppState>,
    Json(payload): Json<NewReportLineItemSerde>,
) -> Result<Json<ReportLineItemSerde>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let non_serde_payload: NewReportLineItem = payload.into();
    let res = match non_serde_payload.insert(database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res.into()))
}

#[axum::debug_handler]
pub async fn get_line_items_by_report(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ReportLineItemSerde>>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportLineItem::get_by_report(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };
    let serde_res: Vec<ReportLineItemSerde> = res.into_iter().map(|i| i.into()).collect();

    Ok(Json(serde_res))
}

#[axum::debug_handler]
pub async fn get_line_item(
    Path(path): Path<(i64, i64)>,
    State(state): State<AppState>,
) -> Result<Json<ReportLineItemSerde>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportLineItem::get_by_path(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res.into()))
}
pub async fn update_line_item<'a>(
    Path(path): Path<(i64, i64)>,
    State(state): State<AppState>,
    Json(payload): Json<NewReportLineItemSerde>,
) -> Result<Json<ReportLineItemSerde>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportLineItem::replace(path, &payload.into(), database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res.into()))
}

#[axum::debug_handler]
pub async fn delete_line_item(
    Path(path): Path<(i64, i64)>,
    State(state): State<AppState>,
) -> Result<Json<ReportLineItemSerde>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportLineItem::delete(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res.into()))
}

#[axum::debug_handler]
pub async fn clear_line_items(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<(), StatusCode> {
    let database_connection = &mut state.get_conn()?;

    if let Err(e) = ReportLineItem::clear_by_report(path, database_connection) {
        log::error!("{e}");
        Err(StatusCode::BAD_GATEWAY)
    } else {
        Ok(())
    }
}
