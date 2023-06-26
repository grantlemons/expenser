use crate::AppState;
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    response::Result,
    Json,
};
use expenser::{NewUser, Report, ReportAccess, User, UserInfo};

#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Result<Json<UserInfo>, StatusCode> {
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
pub async fn get_reports_by_owner(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Report>>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match Report::get_by_owner(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn get_reports_by_view_access(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Report>>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match ReportAccess::get_report_by_borrower(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn get_user(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<UserInfo>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match User::get_by_id(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn update_user(
    Path(path): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Result<Json<UserInfo>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match User::replace(path, &payload, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn delete_user(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<UserInfo>, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match User::delete(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(Json(res))
}

#[axum::debug_handler]
pub async fn clear_users(State(state): State<AppState>) -> Result<(), StatusCode> {
    let database_connection = &mut state.get_conn()?;

    if let Err(e) = User::clear(database_connection) {
        log::error!("{e}");
        Err(StatusCode::BAD_GATEWAY)
    } else {
        Ok(())
    }
}

#[axum::debug_handler]
pub async fn get_profile_picture(
    Path(path): Path<i64>,
    State(state): State<AppState>,
) -> Result<Bytes, StatusCode> {
    let database_connection = &mut state.get_conn()?;

    let res = match User::get_profile_picture(path, database_connection) {
        Ok(res) => res,
        Err(e) => {
            log::error!("{e}");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    Ok(res)
}

#[axum::debug_handler]
pub async fn update_profile_picture(
    Path(path): Path<i64>,
    State(state): State<AppState>,
    payload: Bytes,
) -> Result<(), StatusCode> {
    let database_connection = &mut state.get_conn()?;

    if let Err(e) = User::update_profile_picture(path, &payload, database_connection) {
        log::error!("{e}");
        Err(StatusCode::BAD_GATEWAY)
    } else {
        Ok(())
    }
}

#[derive(serde::Deserialize)]
pub struct Password {
    password: String,
}

#[axum::debug_handler]
pub async fn update_password(
    Path(path): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<Password>,
) -> Result<(), StatusCode> {
    let database_connection = &mut state.get_conn()?;

    if let Err(e) = User::update_password(path, payload.password, database_connection) {
        log::error!("{e}");
        Err(StatusCode::BAD_GATEWAY)
    } else {
        Ok(())
    }
}
