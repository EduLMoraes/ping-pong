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
            let lines: i32 = var("LINES").expect("Erro ao coletar 'LINES'").trim().parse::<i32>().expect("Erro ao converter 'COLUMNS' pra inteiro");
            let columns: i32 = var("COLUMNS").expect("Erro ao coletar 'COLUMNS'").trim().parse::<i32>().expect("Erro ao converter 'COLUMNS' pra inteiro");
            let board: Vec<Vec<char>> = vec![vec![' '; columns as usize]; lines as usize];
            let player: Player = Player::new();
            let scoreboard: Scoreboard = Scoreboard::new();
            let ball: Ball = Ball::new(lines, columns);
            let mut machine: Player = Player::new();
            
            machine.x = columns-2;

            let is_player_win: bool = play(player, machine, ball, scoreboard, board);
            if is_player_win {
                winner();
            }
            else{
                loser();
            }
        }
        else if choice == "B".to_string() || choice == "b".to_string(){
            instruct();
        }
        else{
            break;
        }
    }
}


