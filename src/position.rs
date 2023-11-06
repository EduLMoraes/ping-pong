use crate::structs::Player;
use crate::var;

pub fn position_player(mut player: Player, mut board: Vec<Vec<char>>) -> Vec<Vec<char>>{
    let columns: i32 = board[0].len() as i32;
    let lines: i32 = board.len() as i32;

    for _i in 0..player.width{

        for y in 0..lines{

        if y == player.y{
            if y < player.height{
                board[player.y as usize][player.x as usize] = '|';
                player.y += 1;
            } 
        }

            for x in 0..columns{
                if x == player.x{
                    if y < player.height && x < player.width{
                        board[y as usize][player.x as usize] = '|';
                    }
                }
            }
        }

        player.x += 1;
        player.y = 0;
    }

    board
}