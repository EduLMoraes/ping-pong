use crate::structs::Player;

pub fn position_player(mut player: Player, mut board: Vec<Vec<char>>) -> Vec<Vec<char>>{
    let columns: i32 = board[0].len() as i32;
    let lines: i32 = board.len() as i32;

    if player.x < 0 {
        player.x = 0;
    }
    if player.y < 0{
        player.y = 0;
    }

    if player.width == 1 && player.height == 1 {
        board[player.y as usize][player.x as usize] = '|';
    }

    else{
        for _i in 0..player.width{
    
            if player.x >= columns {
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
    }

    board
}