// Opa galera, safe? Trago um desafio de programação, a idéia do desafio é recriar
// o jogo de ping pong no terminal, quem aí já jogou? Há algumas coisas a levar em
// consideração só como: 
//  - São 2 jogadores, podendo ser player x máquina ou player x player; 
//  - Tem que ter um placar que será atualizado ao realizar um ponto; 
//  - O jogo termina quando um dos jogadores abrir uma vantagem de 3 pontos (opcional).

mod prelude;
use crate::prelude::*;

fn main() {
    loop{
        let choice: String = menu();

        if choice == "A".to_string() || choice == "a".to_string(){
            let lines: i32 = 20;
            let columns: i32 = 40;
            let board: Vec<Vec<i8>> = vec![vec![0; columns as usize]; lines as usize];

            let player: Player = Player::new();
            let machine: Player = Player::new();
            let scoreboard: Scoreboard = Scoreboard::new();
            let ball: Ball = Ball::new(lines, columns);

            let is_player_win: bool = play(player, machine, ball, scoreboard, board);
            if is_player_win {
                winner();
            }
            else{
                loser();
            }
        }
        else{
            break;
        }
    }
}


