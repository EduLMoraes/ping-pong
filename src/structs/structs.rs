pub struct Ball{
    x: i32,
    y: i32,
    speed: i32
}
impl Ball{
    pub fn new(lines: i32, columns: i32) -> Self{
        Ball { x: lines/2, y: columns/2, speed: 1}
    }
}

pub struct Player{
    x: i32,
    y: i32,
    speed: i8,
}
impl Player{
    pub fn new() -> Self{
        Player { x: 0, y: 0, speed: 1 }
    }
}

pub struct Scoreboard{
    home: i8,
    visit: i8
}
impl Scoreboard {
    pub fn new() -> Self{
        Scoreboard {home: 0, visit: 0}
    }
}