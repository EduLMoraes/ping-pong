#[path = "./screens/menu.rs"]
pub mod menu;
pub use menu::*;

#[path = "./screens/board.rs"]
pub mod board;

#[path = "./structs/structs.rs"]
pub mod structs;
pub use structs::*;

#[path = "./game.rs"]
mod game;
pub use game::*;

#[path = "./position.rs"]
mod position;
pub use position::*;
