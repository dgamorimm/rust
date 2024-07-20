use std::io;

fn introdution(){
    println!("Hello, world!");
}

fn variables1(){
    // opcional passar o &str . Isso só informa para o compilador que é uma string
    let name: &str = "Douglas";
    println!("Hello {}!", name);
}

fn variables2(){
    // Para atribuir variáveis de forma dinâmica passamos o prefixo "mut" de muttable ou seja mutável para que receba novos valores
    let mut name: &str = "Douglas";
    name = "Vitória";
    println!("Hello {}!", name);
}

fn datatypes_numbers(){
    // Inteiros
    let _numint: i32 = 23; // Por padrão o Rust assume o tipo i32
    let _numint2: i64 = 20254; // Conseguimos alterar o padrão do inteiro de i8 até i128 bits
    let _numint3: u64 = 2000; // Unsigned, apenas valores positivos
    // Floats
    let _numfloat32: f32 = 23.0; // Números booleanos com 32 bits
    let _numfloat64: f64 = 61561.799; // Números booleanos com 64 bits
    // Booleanos
    let _booltrue: bool = true;
    let _boolfalse: bool = false;
}

fn convert_to_int(data_input: & String) -> i32{
    let x: i32 = data_input.trim().parse::<i32>().unwrap(); // Convertendo uma string para um numero inteiro
    // trim() = ele vai remover os espaços da string
    // parse::<i32>() = vai converter para o numero inteiro
    // unwrap() = ele verifica se o valor vai ser do tipo None e retorna o erro. Em caso de sucesso retorna o valor convertido.
    x // Retornando o valor
}

fn convert(){
    let mut number01: String = String::new();
    io::stdin().read_line(&mut number01).expect("Failed to read line"); // Estamos recebendo o numero 1 e passando a mensagem de erro caso haja falha
    let mut number02: String = String::new();
    io::stdin().read_line(&mut number02).expect("Failed to read line"); // Estamos recebendo o numero 2 e passando a mensagem de erro caso haja falha

    // Chamando a função de conversão para inteiro e passando a condicional
    if convert_to_int(&number01) > convert_to_int(&number02){
        println!("O numero {} é maior que {}", number01, number02);
    }
    else {
        println!("O numero {} é menor ou igual que {}", number01, number02);
    }
}

fn while_looping(){
    let mut soma = 0;
    let mut valor_entrada = String::new();
    io::stdin().read_line(&mut valor_entrada).expect("Failed to read line");
    let mut valor_i32 = convert_to_int(&valor_entrada);
    while valor_i32 != 0 {
        let mut r = valor_i32 % 10;
        soma = soma + r;
        valor_i32 = valor_i32 / 10;
    }
    println!("O valor da soma dos digitos {}", soma);
}

fn functions_double(num : i32) -> i32{
    return 2*num;
}

fn functions_bigger(num1 : i32, num2 : i32) -> i32{
    if num1 >= num2{
        return num1;
    }
    else {
        return num2;
    }
}

fn functions_returns_diffs(par_a : f32, par_b: i128) -> f32{
    println!("Essa função devolve um valor flutuante");
    10 as f32 // faz uma especie de cast do valor ao ser retornado para quem chamar a função. Pode colocar tudo junto tambem 10f32
}

fn for_looping(){

    let animals = vec!["Coelho", "Gato", "Macaco"];

    for animal in animals{
        println!("O animal é {}", animal);
    }

    let range = 20..30;

    for i in range{
        println!("O numero esta variando entre {}", i);
    }
}

fn main() {
    // introdution();
    // variables1();
    // variables2();
    // datatypes_numbers();
    // convert();
    // while_looping();
    // println!("O dobro do numero 5 é {}", functions_double(5));
    // println!("O maior valor entre {} e {} é {}", 5, 10, functions_bigger(5, 10));
    for_looping();
}
