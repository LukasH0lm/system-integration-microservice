use crate::domain::channel::{Channel, ChannelName};
use crate::shared::contracts::ChannelShared;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// This is our Dependency Injection container.
// Arc = Allows multiple threads to share it. Mutex = Prevents data races.
pub type AppState = Arc<Mutex<Vec<Channel>>>;

// POST: Create a new entity
pub async fn create_channel(
    State(state): State<AppState>,
    Json(payload): Json<ChannelShared>,
) -> Result<(StatusCode, Json<ChannelShared>), (StatusCode, String)> {
    // 1. Validate using our Value Object
    let name =
        ChannelName::new(payload.name).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // 2. Create the strict Domain Entity
    let channel = Channel::new(name);

    // 3. Save to our "Database"
    let mut db = state.lock().unwrap();
    db.push(channel.clone());

    // 4. Convert back to the Shared Contract and return 201 Created
    Ok((StatusCode::CREATED, Json(ChannelShared::from(&channel))))
}

// GET: Read multiple entities
pub async fn get_all_channels(State(state): State<AppState>) -> Json<Vec<ChannelShared>> {
    let db = state.lock().unwrap();
    let dtos: Vec<ChannelShared> = db.iter().map(|c| ChannelShared::from(c)).collect();
    Json(dtos)
}

// GET: Read a single entity
pub async fn get_channel(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ChannelShared>, StatusCode> {
    let db = state.lock().unwrap();

    match db.iter().find(|c| c.id() == id) {
        Some(channel) => Ok(Json(ChannelShared::from(channel))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// PUT: Update an entity
pub async fn update_channel(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ChannelShared>,
) -> Result<Json<ChannelShared>, (StatusCode, String)> {
    let mut db = state.lock().unwrap();

    let channel = db
        .iter_mut()
        .find(|c| c.id() == id)
        .ok_or((StatusCode::NOT_FOUND, "Channel not found".to_string()))?;

    let new_name =
        ChannelName::new(payload.name).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Mutate the entity
    channel.rename(new_name);

    Ok(Json(ChannelShared::from(&*channel)))
}

// DELETE: Delete an entity
pub async fn delete_channel(State(state): State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
    let mut db = state.lock().unwrap();
    let initial_len = db.len();

    db.retain(|c| c.id() != id); // Removes the channel if the ID matches

    if db.len() < initial_len {
        StatusCode::NO_CONTENT // 204 Success, no body
    } else {
        StatusCode::NOT_FOUND
    }
}
