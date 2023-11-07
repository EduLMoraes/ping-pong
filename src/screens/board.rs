use crate::structs::*;
use crate::var;

pub fn most_score(score: &Scoreboard){
    if cfg!(target_os = "windows"){
        std::process::Command::new("cls").status().unwrap();
    } else{
        std::process::Command::new("clear").status().unwrap();
    }
    
    println!(".______________.");
    println!("|     {} X {}    |", score.home, score.visit);
}

pub fn most_board(board: &Vec<Vec<char>>){


    print!(".");
    for _i in 0..var("COLUMNS")
                    .expect("Erro ao coletar 'COLUMNS'")
                    .trim()
                    .parse::<i32>()
                    .expect("Erro ao converter para inteiro")
    {
        print!("_");
    }
    print!(".\n");

    for i in 0..var("LINES")
                    .expect("Erro ao coletar 'LINES'")
                    .trim()
                    .parse::<i32>()
                    .expect("Erro ao converter para inteiro")
    {
        print!("|");

        for j in 0..var("COLUMNS")
                        .expect("Erro ao coletar 'COLUMNS'")
                        .trim()
                        .parse::<i32>()
                        .expect("Erro ao converter para inteiro")
        {
            print!("{}", board[i as usize][j as usize]);
        }

        print!("|\n");
    }

    print!("|");
    for _i in 0..var("COLUMNS")
                    .expect("Erro ao coletar 'COLUMNS'")
                    .trim()
                    .parse::<i32>()
                    .expect("Erro ao converter para inteiro")
    {
        print!("=");
    }
    print!("|\n");

}

pub fn clean_board(columns: usize, lines: usize) -> Vec<Vec<char>>{
    vec![
        vec![
            ' '; 
            columns
        ];
        lines as usize
    ]
}