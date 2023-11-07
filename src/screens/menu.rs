use std::io::stdin;

#[allow(dead_code)]
pub fn menu() -> String{

    let mut choice: String = String::new();

    println!("Bem-vindo");
    println!("Ao ping pong em rust!");
    println!("Escolha uma das opções abaixo:");
    println!("A) Jogar");
    println!("B) Instruções");
    println!("C) Sair");

    stdin().read_line(&mut choice).expect("Erro ao ler escolha!");
    choice.trim().to_string()
}

#[allow(dead_code)]
pub fn instruct(){
    if cfg!(target_os = "windows"){
        std::process::Command::new("cls").status().unwrap();
    } else{
        std::process::Command::new("clear").status().unwrap();
    }

    println!("\n\n====INSTRUÇÕES====");
    println!("\n\nPlayer 1:
        1- Fica no canto esquerdo.
        2- Move com as setas '↓' e '̉↑'."
        );
    println!("\nPlayer 2:
        1- Fica no canto direito.
        2- Move com as teclas 'w' e 's'."
    );
    println!("Vence aquele que obtiver vantagem de 3 pontos.\n\n");
}

#[allow(dead_code)]
pub fn winner(){
    println!("É TETRAAAAA O PLAYER 1 VENCEU!!!");
    println!("CHUUUUUUUUUUUUUUUUUPPAAAAAAAAAAAA PLAYER 2!!");
}

#[allow(dead_code)]
pub fn loser(){
    println!("É TETRAAAAA O PLAYER 2 VENCEU!!!");
    println!("CHUUUUUUUUUUUUUUUUUPPAAAAAAAAAAAA PLAYER 1!!");
}