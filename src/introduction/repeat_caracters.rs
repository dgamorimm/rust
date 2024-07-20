fn tem_caracteres_unicos(input: &str) -> bool {
    let mut conjunto_de_caracteres = [false; 128]; // cria um array de booleanos de 128 posicoes repetindo o false 128 vezes

    for &c in input.as_bytes() {
        let indice = c as usize; // casting de bytes para um indice de um arayy
        println!("Caractere: {}, Indice : {}", c as char, indice);
        if conjunto_de_caracteres[indice]{
            println!("Caractere repetido");
            return false
        }
        conjunto_de_caracteres[indice] = true;
    }

    return true  //tods os caracteres são unicos

}


fn main() {

    println!("Digite uma palavra");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if tem_caracteres_unicos(&input){
        println!("Os caracteres do texto {} são unicos", input)
    }
    else {
        println!("Os caracteres do texto {} não são unicos", input)
    }
}