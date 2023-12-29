use crate::board::*;
use crate::structs::*;
use crate::{move_ball, position_player};
use k_board::{keyboard::Keyboard, keys::Keys::Char, keys::Keys::Down, keys::Keys::Up};

#[allow(dead_code)]
pub fn play(mut ball: Ball, mut score: Scoreboard, mut board: Vec<Vec<char>>) -> bool {
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

    let len = board[0].len() as i32;
    let columns = len;
    let lines = board.len() as i32;

    let (tmp_ball_x, tmp_ball_y) = (ball.x, ball.y);

    println!("alo");
    let mut player1 = get_player1_instance();
    let mut player2 = get_player2_instance();

    player2.x = 69;

    loop {
        let mut key = Keyboard::new();

        if ball.x >= 35 {
            match key.read_key() {
                Char('w') => player2.up(),
                Char('s') => {
                    if player2.height < lines {
                        player2.down()
                    }
                }
                _ => {}
            }
        } else {
            match key.read_key() {
                Up => player1.up(),
                Down => {
                    if player1.height < lines {
                        player1.down()
                    }
                }
                Char('w') => player2.up(),
                Char('s') => {
                    if player2.height < lines {
                        player2.down()
                    }
                }
                _ => {}
            }
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
