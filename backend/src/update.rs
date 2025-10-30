use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

#[derive(Deserialize)]
pub struct UpdateHoursRequest {
    pub state_id: String,
    pub hours: i32,
}

#[derive(Serialize)]
pub struct UpdateHoursResponse {
    pub hours_complete: i32,
}

pub async fn update_hours(
    state: State<crate::AppState>,
    cookies: Cookies,
    Json(data): Json<UpdateHoursRequest>,
) -> Result<Json<UpdateHoursResponse>, (StatusCode, &'static str)> {
    // Get session token from cookie
    let session_token = cookies
        .get("session")
        .ok_or((StatusCode::UNAUTHORIZED, "Not logged in"))?
        .value()
        .to_string();

    // Verify session and get user ID
    let user = entity::user::Entity::find()
        .inner_join(entity::session::Entity)
        .filter(entity::session::Column::Token.eq(session_token))
        .one(&state.conn)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or((StatusCode::FORBIDDEN, "Not logged in"))?;

    // Validate hours is non-negative
    if data.hours < 0 {
        return Err((StatusCode::BAD_REQUEST, "Hours cannot be negative"));
    }

    // Look up the state by its code to get the numeric ID
    let state_record = entity::state::Entity::find()
        .filter(entity::state::Column::Name.eq(&data.state_id))
        .one(&state.conn)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or((StatusCode::NOT_FOUND, "State code not found"))?;

    // Find the user_state entry for this user and state
    let user_state_entry = entity::user_state::Entity::find()
        .filter(entity::user_state::Column::UserId.eq(user.id))
        .filter(entity::user_state::Column::StateId.eq(state_record.id))
        .one(&state.conn)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or((StatusCode::NOT_FOUND, "State not found for user"))?;

    // Update the hours_complete field
    let mut user_state_active: entity::user_state::ActiveModel = user_state_entry.into();
    user_state_active.hours_complete = Set(data.hours);

    let updated = user_state_active
        .update(&state.conn)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update hours"))?;

    Ok(Json(UpdateHoursResponse {
        hours_complete: updated.hours_complete,
    }))
}
