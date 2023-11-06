use crate::structs::*;
use crate::board::*;
use crate::position_player;

pub fn play(player1: Player, player2: Player, ball: Ball, score: Scoreboard, mut board: Vec<Vec<char>>) -> bool{
    board = position_player(player1, board);
    board = position_player(player2, board);

    most_score(score);
    most_board(board);
    true
}