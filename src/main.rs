mod prelude;
use crate::prelude::*;

fn main() {
    loop {
        let choice: String = menu();

        if choice == *"A" || choice == *"a" {
            let lines: i32 = 20; //var("LINES").expect("Erro ao coletar 'LINES'").trim().parse::<i32>().expect("Erro ao converter 'COLUMNS' pra inteiro");
            let columns: i32 = 70; //var("COLUMNS").expect("Erro ao coletar 'COLUMNS'").trim().parse::<i32>().expect("Erro ao converter 'COLUMNS' pra inteiro");
            let board: Vec<Vec<char>> = vec![vec![' '; columns as usize]; lines as usize];
            let player: Player = Player::new();
            let scoreboard: Scoreboard = Scoreboard::new();
            let ball: Ball = Ball::new(lines, columns);
            let mut machine: Player = Player::new();

            machine.x = columns - 2;

            let is_player_win: bool = play(player, machine, ball, scoreboard, board);
            if is_player_win {
                winner();
            } else {
                loser();
            }
        } else if choice == *"B" || choice == *"b" {
            instruct();
        } else {
            break;
        }
    }
}
