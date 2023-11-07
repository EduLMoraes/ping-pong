use crate::var;

pub struct Ball{
    pub x: i32,
    pub y: i32,
    pub ray: i32,
    pub speedx: i32,
    pub speed: i32,
    pub speedy: i32
}

#[allow(dead_code)]
impl Ball{
    pub fn new(lines: i32, columns: i32) -> Self{
        Ball { 
            x: columns/2, 
            y: lines/2,
            ray: 3, 
            speedx: 1,
            speed: 1,
            speedy: 1
        }
    }
}

pub struct Player{
    pub x: i32,
    pub y: i32,
    pub speed: i32,
    pub height: i32,
    pub width: i32,
}
#[allow(dead_code)]
impl Player{
    pub fn new() -> Self{
        Player { 
            x: 0,
            y: 0,
            speed: 1,
            height: if var("LINES")
            .expect("Erro ao coletar 'LINES'")
            .trim()
            .parse::<i32>()
            .expect("Erro ao converter para inteiro")%3 == 0{
                var("LINES")
                .expect("Erro ao coletar 'LINES'")
                .trim()
                .parse::<i32>()
                .expect("Erro ao converter para inteiro") / 3
            } else { 5 },
            width: 2
        }
    }
}

pub struct Scoreboard{
    pub home: i8,
    pub visit: i8
}
#[allow(dead_code)]
impl Scoreboard {
    pub fn new() -> Self{
        Scoreboard {
            home: 0, 
            visit: 0
        }
    }
}