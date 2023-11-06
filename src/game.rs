use crate::structs::*;
use crate::board::*;
use crate::position_player;

pub fn play(mut player1: Player, mut player2: Player, ball: Ball, score: Scoreboard, mut board: Vec<Vec<char>>) -> bool{
    loop{
        
        (player1, board) = position_player(player1, board);
        //(player2, board) = position_player(player2, board);
    
        most_score(&score);
        most_board(&board);

        if score.home-3 == score.visit{
            break;
        }
    }
    true
}