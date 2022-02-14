#[macro_use]
extern crate rocket;

use dashmap::DashMap;
use rocket::{
    serde::{json::Json, Deserialize},
    State,
};
use uuid::Uuid;

mod game;

use game::TicTacToe;

type GameStorage = DashMap<Uuid, TicTacToe>;

#[post("/create")]
fn create_game(games: &State<GameStorage>) -> String {
    let game_id = loop {
        let id = Uuid::new_v4();
        if !games.contains_key(&id) {
            break id;
        }
    };
    games.insert(game_id, TicTacToe::new());
    game_id.to_string()
}

#[derive(Deserialize)]
struct MoveInfo {
    x: u8,
    y: u8,
}

#[post("/<game_id>/play", data = "<move_info>")]
fn place_symbol(
    game_id: Uuid,
    move_info: Json<MoveInfo>,
    games: &State<GameStorage>,
) -> Result<String, &'static str> {
    let MoveInfo { x, y } = move_info.0;
    let (won, active_player) = {
        let mut board = games.get_mut(&game_id).ok_or("invalid game id")?;
        let current_player = board.active_player();
        (
            board.place_cell(x, y).map_err(|err| err.message())?,
            current_player,
        )
    };
    Ok(if won {
        games.remove(&game_id);
        format!("{} won!", active_player.name())
    } else {
        String::new()
    })
}

#[get("/<game_id>/field")]
fn get_field(game_id: Uuid, games: &State<GameStorage>) -> Json<Option<TicTacToe>> {
    Json(games.get(&game_id).as_deref().cloned())
}

#[post("/ping")]
fn ping() -> &'static str {
    "rocket pong!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_game, place_symbol, get_field, ping])
        .manage(GameStorage::new())
}
