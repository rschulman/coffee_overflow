use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect};
use serde::Serialize;
use tower_cookies::Cookies;

#[derive(Serialize)]
pub struct StateHours {
    state_code: String,
    hours_complete: u16,
    legal_hours: u16,
}

#[derive(Serialize)]
pub struct UserDetailsResponse {
    states: Vec<StateHours>,
}

pub async fn user_details(
    state: State<crate::AppState>,
    cookies: Cookies,
) -> Result<Json<UserDetailsResponse>, (StatusCode, &'static str)> {
    // Get session token from cookie
    let session_token = cookies
        .get("session")
        .ok_or((StatusCode::UNAUTHORIZED, "Not logged in"))?
        .value()
        .to_string();

    let resp = entity::user::Entity::find()
        .inner_join(entity::session::Entity)
        .filter(entity::session::Column::Token.eq(session_token))
        .one(&state.conn)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or((StatusCode::FORBIDDEN, "Not logged in"))?;

    #[derive(FromQueryResult)]
    struct QueryRes {
        name: String,
        legal_hours: u16,
        hours_complete: u16,
    }

    let hours = entity::state::Entity::find()
        .inner_join(entity::user_state::Entity)
        .filter(entity::user_state::Column::UserId.eq(resp.id))
        .select_only()
        .column(entity::user_state::Column::HoursComplete)
        .column(entity::state::Column::LegalHours)
        .column(entity::state::Column::Name)
        .into_model::<QueryRes>()
        .all(&state.conn)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?;

    let states_response = hours
        .iter()
        .map(|h| StateHours {
            state_code: h.name.clone(),
            hours_complete: h.hours_complete,
            legal_hours: h.legal_hours,
        })
        .collect();

    Ok(Json(UserDetailsResponse {
        states: states_response,
    }))
}
