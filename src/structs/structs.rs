use crate::var;

pub struct Ball{
    pub x: i32,
    pub y: i32,
    pub speed: i32
}
impl Ball{
    pub fn new(lines: i32, columns: i32) -> Self{
        Ball { 
            x: lines/2, 
            y: columns/2, 
            speed: 1
        }
    }
}

pub struct Player{
    pub x: i32,
    pub y: i32,
    pub speed: i8,
    pub height: i32,
    pub width: i32,
}
impl Player{
    pub fn new() -> Self{
        Player { 
            x: 0,
            y: 0,
            speed: 1,
            height: var("LINES")
            .expect("Erro ao coletar 'LINES'")
            .trim()
            .parse::<i32>()
            .expect("Erro ao converter para inteiro")/3,
            width: 2
        }
    }
}

pub struct Scoreboard{
    pub home: i8,
    pub visit: i8
}
impl Scoreboard {
    pub fn new() -> Self{
        Scoreboard {
            home: 0, 
            visit: 0
        }
    }
}