use std::io::stdin;

pub fn menu() -> String{

    let mut choice: String = String::new();

    println!("Bem-vindo");
    println!("Ao ping pong em rust!");
    println!("Escolha uma das opções abaixo:");
    println!("A) Jogar");
    println!("B) Sair");

    stdin().read_line(&mut choice).expect("Erro ao ler escolha!");
    choice.trim().to_string()
}

pub fn winner(){
    
}
pub fn loser(){}