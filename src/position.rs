use crate::structs::{Player, Ball};

pub fn position_player(mut player: Player, mut board: Vec<Vec<char>>) -> (Player, Vec<Vec<char>>){
    if board.len() <= 0 {
        return (player, board);
    }

    let columns = board[0].len() as i32;
    let lines = board.len() as i32;

    player.x = player.x.max(0);
    player.y = player.y.max(0);

    if player.width == 1 && player.height == 1 {
        board[player.y as usize][player.x as usize] = '|';
    }
    else{
        let tmp_x = player.x;
        let tmp_y = player.y;

        for _i in 0..player.width{
    
            if player.x >= columns || player.x >= player.width{
                player.x -= 1;
            }

            let y_init = player.y;
    
            for y in 0..lines{
                if y == player.y{
                    if y < player.height{
                        board[player.y as usize][player.x as usize] = '|';
                        player.y += 1;
                    } 
                }
    
                for x in 0..columns{
                    if x == player.x && y == player.y{
                        if y < player.height && x < player.width{
                            board[y as usize][player.x as usize] = '|';
                        }
                    }
                }
            }
    
            player.x += 1;
            if player.y >= lines || player.y >= player.height{
                player.y -= player.height;
            }
            if player.y != y_init{
                player.y = y_init;
            }
        
        }
        player.y = tmp_y;
        player.x = tmp_x;
    }

    (player, board)
}

pub fn move_ball(mut ball: Ball, mut board: Vec<Vec<char>>) -> (Ball, Vec<Vec<char>>){
    
    (ball, board)
}