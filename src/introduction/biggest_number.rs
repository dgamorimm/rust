
fn maior_numero(numeros: &[i32]) -> i32 {
    let mut maior = numeros[0];
    for num in numeros{
        if num > &maior {   // fazer referencia com referencia temos que passar o &
            maior = *num;  // agora a variavel maior quero que seja o ownership, para desreferenciar passa o *
        }
    }

    return maior;
}

fn main() {
    let numeros = [1, 5, 2, 8, 9, 3];
    let maior = maior_numero(&numeros);  // criando um areferencia aos dados numeros é dona &numeros (isso economiza a memoria pois trabalha diretamente com os dados originais)

    println!("O maior número é {}", maior);
}