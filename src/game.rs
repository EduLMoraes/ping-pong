use crate::board::*;
use crate::structs::*;
use crate::{move_ball, position_player};
use k_board::{keyboard::Keyboard, keys::Keys};

#[allow(dead_code)]
pub fn play(
    mut player1: Player,
    mut player2: Player,
    mut ball: Ball,
    mut score: Scoreboard,
    mut board: Vec<Vec<char>>,
) -> bool {
    /* let lines = var("LINES")
        .expect("Erro ao coletar 'LINES'")
        .trim()
        .parse::<i32>()
        .expect("Erro ao converter 'COLUMNS' pra inteiro");

    let columns = var("COLUMNS")
        .expect("Erro ao coletar 'COLUMNS'")
        .trim()
        .parse::<usize>()
        .expect("Erro ao converter 'COLUMNS' pra inteiro");*/

    let columns = board[0].len() as i32;
    let lines = board.len() as i32;

    let (tmp_ball_x, tmp_ball_y) = (ball.x, ball.y);

    loop {
        let mut key = Keyboard::new();

        match key.read_key() {
            Keys::Down => {
                if player1.height < lines {
                    player1.y += player1.speed;
                    player1.height += player1.speed;
                }
            }
            Keys::Up => {
                player1.y -= player1.speed;
                if player1.y >= 0 {
                    player1.height -= player1.speed;
                }
            }
            Keys::Char('s') => {
                if player2.height < lines {
                    player2.y += player2.speed;
                    player2.height += player2.speed;
                }
            }
            Keys::Char('w') => {
                player2.y -= player2.speed;
                if player2.y >= 0 {
                    player2.height -= player2.speed;
                }
            }
            _ => {}
        }

        (ball, board) = move_ball(ball, board);
        (player1, board) = position_player(player1, board);
        (player2, board) = position_player(player2, board);

        if ball.y == lines - 1 || ball.y == 0 {
            ball.speedy = -ball.speedy;
        }

        if board[ball.y as usize][ball.x as usize] == '|' {
            ball.speedx = -ball.speedx;
            ball.speed += 1;
        } else if ball.x == columns - 1 {
            score.home += 1;
            ball.speedx = -ball.speedx;
            ball.x = tmp_ball_x;
            ball.y = tmp_ball_y;
        } else if ball.x == 0 {
            score.visit += 1;
            ball.speedx = -ball.speedx;
            ball.x = tmp_ball_x;
            ball.y = tmp_ball_y;
        }

        ball.x += ball.speedx;
        ball.y += ball.speedy;

        most_score(&score);
        most_board(&board);

        board = clean_board(columns as usize, lines as usize);

        if score.home - 3 == score.visit {
            return true;
        } else if score.home == score.visit - 3 {
            return false;
        }
    }
}
