use crate::structs::*;
use crate::board::*;
use crate::position_player;
use crate::var;
use k_board::{Keyboard, Keys::Up, Keys::Down};

pub fn play(mut player1: Player, mut player2: Player, ball: Ball, score: Scoreboard, mut board: Vec<Vec<char>>) -> bool{
    let lines = var("LINES")
        .expect("Erro ao coletar 'LINES'")
        .trim()
        .parse::<i32>()
        .expect("Erro ao converter 'COLUMNS' pra inteiro");
    
    let columns = var("COLUMNS")
        .expect("Erro ao coletar 'COLUMNS'")
        .trim()
        .parse::<usize>()
        .expect("Erro ao converter 'COLUMNS' pra inteiro");

    loop{
        
        for key in Keyboard::new() {
            match key {
                Down => {
                    if player1.height < lines{
                        player1.y += player1.speed;
                        player1.height += player1.speed;
                    }
                    break
                },
                Up => {
                    player1.y -= player1.speed; 
                    if player1.y >= 0{
                        player1.height -= player1.speed;
                    }
                    break
                },
                _ => break
            }
        }

        (player1, board) = position_player(player1, board);
        (player2, board) = position_player(player2, board);

        most_score(&score);
        most_board(&board);

        board = vec![
            vec![
                ' '; 
                columns
            ];
            lines as usize
        ];

        if score.home-3 == score.visit{
            break;
        }
    }
    true
}