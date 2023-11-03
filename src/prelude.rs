#[path = "./screens/menu.rs"]
pub mod menu;
pub use menu::*;

#[path = "./structs/structs.rs"]
pub mod structs;
pub use structs::*;

#[path = "./game.rs"]
mod game;
pub use game::*;