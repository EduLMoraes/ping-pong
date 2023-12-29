use crate::Scoreboard;

pub fn most_score(score: &Scoreboard) {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cls").status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }

    println!(".______________.");
    println!("|     {} X {}    |", score.home, score.visit);
}

pub fn most_board(board: &[Vec<char>]) {
    print!(".");
    for _i in 0..board[0].len() {
        print!("_");
    }
    println!(".");

    for i in 0..board.len() {
        print!("|");

        for j in 0..board[0].len() {
            if j == board[0].len() / 2 {
                print!("|");
            }
            print!("{}", board[i][j]);
        }

        println!("|");
    }

    print!("|");
    for _i in 0..board[0].len() {
        print!("=");
    }
    println!("|");
}

pub fn clean_board(columns: usize, lines: usize) -> Vec<Vec<char>> {
    vec![vec![' '; columns]; lines]
}
