pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub ray: i32,
    pub speedx: i32,
    pub speed: i32,
    pub speedy: i32,
}

#[allow(dead_code)]
impl Ball {
    pub fn new(lines: i32, columns: i32) -> Self {
        Ball {
            x: columns / 2,
            y: lines / 2,
            ray: 3,
            speedx: 1,
            speed: 1,
            speedy: 1,
        }
    }
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub speed: i32,
    pub height: i32,
    pub width: i32,
}
#[allow(dead_code)]
impl Player {
    pub fn new() -> Self {
        Player {
            x: 0,
            y: 0,
            speed: 1,
            height: 5,
            // height: if var("LINES")
            // .expect("Erro ao coletar 'LINES'")
            // .trim()
            // .parse::<i32>()
            // .expect("Erro ao converter para inteiro")%3 == 0{
            //     var("LINES")
            //     .expect("Erro ao coletar 'LINES'")
            //     .trim()
            //     .parse::<i32>()
            //     .expect("Erro ao converter para inteiro") / 3
            // } else { 5 },
            width: 2,
        }
    }

    pub fn up(&mut self){
        self.y -= self.speed;
        if self.y >= 0 {
            self.height -= self.speed;
        }

    }

    pub fn down(&mut self){
        self.y += self.speed;
        self.height += self.speed;
    }
}

pub use lazy_static::lazy_static;
pub use std::sync::Mutex;
lazy_static! {
    static ref PLAYER1: Mutex<Player> = Mutex::new(Player::new());
}
lazy_static! {
    static ref PLAYER2: Mutex<Player> = Mutex::new(Player::new());
}

#[allow(dead_code)]
pub fn get_player1_instance() -> std::sync::MutexGuard<'static, Player> {
    PLAYER1.lock().unwrap()
}
#[allow(dead_code)]
pub fn get_player2_instance() -> std::sync::MutexGuard<'static, Player> {
    PLAYER2.lock().unwrap()
}

pub struct Scoreboard {
    pub home: i8,
    pub visit: i8,
}
#[allow(dead_code)]
impl Scoreboard {
    pub fn new() -> Self {
        Scoreboard { home: 0, visit: 0 }
    }
}
