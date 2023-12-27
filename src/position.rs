use crate::structs::{Ball, Player};

pub fn position_player(mut player: Player, mut board: Vec<Vec<char>>) -> (Player, Vec<Vec<char>>) {
    if board.is_empty() {
        return (player, board);
    }

    let columns = board[0].len() as i32;
    let lines = board.len() as i32;

    player.x = player.x.max(0);
    player.y = player.y.max(0);

    if player.width == 1 && player.height == 1 {
        board[player.y as usize][player.x as usize] = '|';
    } else {
        let tmp_x = player.x;
        let tmp_y = player.y;

        for _i in 1..=player.width {
            if player.x >= columns || player.x >= player.width {
                player.x -= 1;
            }

            let y_init = player.y;

            for y in 0..lines {
                if y == player.y && y < player.height {
                    board[player.y as usize][player.x as usize] = '|';
                    player.y += 1;
                }

                for x in 0..columns {
                    if x == player.x && y == player.y && y < player.height && x < player.width {
                        board[y as usize][player.x as usize] = '|';
                    }
                }
            }

            player.x += 1;
            if player.y >= lines || player.y >= player.height {
                player.y -= player.height;
            }
            if player.y != y_init {
                player.y = y_init;
            }
        }
        player.y = tmp_y;
        player.x = tmp_x;
    }

    (player, board)
}

#[allow(dead_code)]
pub fn move_ball(mut ball: Ball, mut board: Vec<Vec<char>>) -> (Ball, Vec<Vec<char>>) {
    if board.is_empty() {
        return (ball, board);
    }

    ball.x = ball.x.max(0);
    ball.y = ball.y.max(0);

    board[ball.y as usize][ball.x as usize] = '@';

    (ball, board)
}
