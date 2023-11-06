use crate::structs::Player;
use crate::var;

pub fn position_player(player: Player, mut board: Vec<Vec<char>>) -> Vec<Vec<char>>{
    for y in 0..var("LINES")
    .expect("Erro ao coletar LINES")
    .trim()
    .parse::<i32>()
    .expect("Erro ao transformar em inteiro"){

        for x in 0..var("COLUMNS")
        .expect("Erro ao coletar COLUMNS")
        .trim()
        .parse::<i32>()
        .expect("Erro ao transformar em inteiro"){
            if y == player.y || x == player.x{
                if y < player.heigth && x < player.width{
                    board[y as usize][x as usize] = '|';
                } 
            }
        }
    }

    board
}