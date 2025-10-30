use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_with::DeserializeFromStr;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct RegisterData {
    username: String,
    password: String,
    fullname: String,
    states: HashMap<UsState, HourRequirements>,
}

#[derive(Deserialize)]
pub struct HourRequirements {
    completed: u16,
    due: chrono::NaiveDate,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    message: String,
    user_id: i32,
}

pub async fn register(
    state: State<crate::AppState>,
    Json(data): Json<RegisterData>,
) -> Result<Json<RegisterResponse>, (StatusCode, &'static str)> {
    // Check if username already exists
    let existing_user = entity::user::Entity::find()
        .filter(entity::user::Column::Username.eq(&data.username))
        .one(&state.conn)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?;

    if existing_user.is_some() {
        return Err((StatusCode::CONFLICT, "Username already exists"));
    }

    // Salt and hash the password
    let salted = format!("{}{}", data.password, crate::SALT);
    let mut hasher = Sha256::new();
    hasher.update(salted.as_bytes());
    let hashed_password = format!("{:x}", hasher.finalize());

    // Create new user
    let new_user = entity::user::ActiveModel {
        username: Set(data.username),
        password: Set(hashed_password),
        fullname: Set(data.fullname),
        ..Default::default()
    };

    let user = new_user
        .insert(&state.conn)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user"))?;

    // Link user to states
    link_user_to_states(&state.conn, user.id, data.states)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to link states"))?;

    Ok(Json(RegisterResponse {
        message: "User registered successfully".to_string(),
        user_id: user.id,
    }))
}

async fn link_user_to_states(
    conn: &DatabaseConnection,
    user_id: i32,
    state_names: HashMap<UsState, HourRequirements>,
) -> Result<(), sea_orm::DbErr> {
    use sea_orm::ConnectionTrait;
    use sea_orm::Statement;

    for state in state_names {
        // Find the state by name
        let state_query = format!("SELECT id FROM state WHERE name = '{}' LIMIT 1", state.0);

        let state_result = conn
            .query_one(Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                state_query,
            ))
            .await?;

        if let Some(state_row) = state_result {
            let state_id: i32 = state_row.try_get("", "id")?;

            // Insert into user_state table with hours_complete and renewal_date
            let insert_query = format!(
                "INSERT INTO user_state (user_id, state_id, hours_complete, renewal_date) VALUES ({}, {}, {}, '{}')",
                user_id, state_id, state.1.completed, state.1.due
            );

            conn.execute(Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,
                insert_query,
            ))
            .await?;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeserializeFromStr)]
pub enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

#[derive(Debug)]
pub struct ParseStateError;

impl fmt::Display for ParseStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid state code")
    }
}

impl std::error::Error for ParseStateError {}

impl FromStr for UsState {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AL" => Ok(UsState::Alabama),
            "AK" => Ok(UsState::Alaska),
            "AZ" => Ok(UsState::Arizona),
            "AR" => Ok(UsState::Arkansas),
            "CA" => Ok(UsState::California),
            "CO" => Ok(UsState::Colorado),
            "CT" => Ok(UsState::Connecticut),
            "DE" => Ok(UsState::Delaware),
            "FL" => Ok(UsState::Florida),
            "GA" => Ok(UsState::Georgia),
            "HI" => Ok(UsState::Hawaii),
            "ID" => Ok(UsState::Idaho),
            "IL" => Ok(UsState::Illinois),
            "IN" => Ok(UsState::Indiana),
            "IA" => Ok(UsState::Iowa),
            "KS" => Ok(UsState::Kansas),
            "KY" => Ok(UsState::Kentucky),
            "LA" => Ok(UsState::Louisiana),
            "ME" => Ok(UsState::Maine),
            "MD" => Ok(UsState::Maryland),
            "MA" => Ok(UsState::Massachusetts),
            "MI" => Ok(UsState::Michigan),
            "MN" => Ok(UsState::Minnesota),
            "MS" => Ok(UsState::Mississippi),
            "MO" => Ok(UsState::Missouri),
            "MT" => Ok(UsState::Montana),
            "NE" => Ok(UsState::Nebraska),
            "NV" => Ok(UsState::Nevada),
            "NH" => Ok(UsState::NewHampshire),
            "NJ" => Ok(UsState::NewJersey),
            "NM" => Ok(UsState::NewMexico),
            "NY" => Ok(UsState::NewYork),
            "NC" => Ok(UsState::NorthCarolina),
            "ND" => Ok(UsState::NorthDakota),
            "OH" => Ok(UsState::Ohio),
            "OK" => Ok(UsState::Oklahoma),
            "OR" => Ok(UsState::Oregon),
            "PA" => Ok(UsState::Pennsylvania),
            "RI" => Ok(UsState::RhodeIsland),
            "SC" => Ok(UsState::SouthCarolina),
            "SD" => Ok(UsState::SouthDakota),
            "TN" => Ok(UsState::Tennessee),
            "TX" => Ok(UsState::Texas),
            "UT" => Ok(UsState::Utah),
            "VT" => Ok(UsState::Vermont),
            "VA" => Ok(UsState::Virginia),
            "WA" => Ok(UsState::Washington),
            "WV" => Ok(UsState::WestVirginia),
            "WI" => Ok(UsState::Wisconsin),
            "WY" => Ok(UsState::Wyoming),
            _ => Err(ParseStateError),
        }
    }
}

impl fmt::Display for UsState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            UsState::Alabama => "AL",
            UsState::Alaska => "AK",
            UsState::Arizona => "AZ",
            UsState::Arkansas => "AR",
            UsState::California => "CA",
            UsState::Colorado => "CO",
            UsState::Connecticut => "CT",
            UsState::Delaware => "DE",
            UsState::Florida => "FL",
            UsState::Georgia => "GA",
            UsState::Hawaii => "HI",
            UsState::Idaho => "ID",
            UsState::Illinois => "IL",
            UsState::Indiana => "IN",
            UsState::Iowa => "IA",
            UsState::Kansas => "KS",
            UsState::Kentucky => "KY",
            UsState::Louisiana => "LA",
            UsState::Maine => "ME",
            UsState::Maryland => "MD",
            UsState::Massachusetts => "MA",
            UsState::Michigan => "MI",
            UsState::Minnesota => "MN",
            UsState::Mississippi => "MS",
            UsState::Missouri => "MO",
            UsState::Montana => "MT",
            UsState::Nebraska => "NE",
            UsState::Nevada => "NV",
            UsState::NewHampshire => "NH",
            UsState::NewJersey => "NJ",
            UsState::NewMexico => "NM",
            UsState::NewYork => "NY",
            UsState::NorthCarolina => "NC",
            UsState::NorthDakota => "ND",
            UsState::Ohio => "OH",
            UsState::Oklahoma => "OK",
            UsState::Oregon => "OR",
            UsState::Pennsylvania => "PA",
            UsState::RhodeIsland => "RI",
            UsState::SouthCarolina => "SC",
            UsState::SouthDakota => "SD",
            UsState::Tennessee => "TN",
            UsState::Texas => "TX",
            UsState::Utah => "UT",
            UsState::Vermont => "VT",
            UsState::Virginia => "VA",
            UsState::Washington => "WA",
            UsState::WestVirginia => "WV",
            UsState::Wisconsin => "WI",
            UsState::Wyoming => "WY",
        };
        write!(f, "{}", code)
    }
}
