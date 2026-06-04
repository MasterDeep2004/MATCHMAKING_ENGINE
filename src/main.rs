use axum::{
    routing::{post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct PlayerRequest {
    skill: u8,
}

#[derive(Clone, Debug)]
struct Player {
    id: Uuid,
    skill: u8,
}

struct MatchmakingState {
    players: RwLock<Vec<Player>>,
}

impl MatchmakingState {
    fn new() -> Self {
        Self {
            players: RwLock::new(Vec::new()),
        }
    }

    async fn add_player(&self, skill: u8) -> Player {
        let player = Player {
            id: Uuid::new_v4(),
            skill,
        };
        self.players.write().await.push(player.clone());
        player
    }

    async fn find_match(&self) -> Option<Vec<Player>> {
        let mut players = self.players.write().await;
        if players.len() >= 10 {
            let selected: Vec<_> = players.drain(0..10).collect();
            Some(selected)
        } else {
            None
        }
    }
}#[tokio::main]
async fn main() {
    let state = Arc::new(MatchmakingState::new());

    let app = Router::new()
        .route("/add_player", post(add_player))
        .route("/get_match", post(get_match))
        .with_state(state);

    println!("Server listening on http://0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn add_player(
    Json(payload): Json<PlayerRequest>,
    state: Arc<MatchmakingState>,
) -> Json<Player> {
    let player = state.add_player(payload.skill).await;
    Json(player)
}

async fn get_match(
    state: Arc<MatchmakingState>,
) -> Json<Option<Vec<Player>>> {
    let match_players = state.find_match().await;
    Json(match_players)
}
