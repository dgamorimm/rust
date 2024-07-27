
fn main() {

    let mut  minhaString = String::from("Olá meu nome é Douglas. ");

    println!("O tamanho da minha string é {}", minhaString.len());
    println!("A minha string está vazia ? {}", minhaString.is_empty());

    for token in minhaString.split_whitespace(){
        println!("{}", token);
    }

    println!("O nome Douglas está contido na string ? {}", minhaString.contains("Douglas"));

    minhaString.push_str("Bem-vindo a aula."); // concatenação
    println!("{}", minhaString);
}