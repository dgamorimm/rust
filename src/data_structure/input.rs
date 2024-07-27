use std::io;

fn main(){

    let mut input = String::new();

    println!("Digite uma palavra:\n\n");

    match io::stdin().read_line(&mut input){
        Ok(_) => println!("Voce digitou: {}", input.to_uppercase()),
        Err(e) => println!("Erro: {}", e)
        
    }
}