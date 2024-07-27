fn calcula_media(notas: &[f32]) -> f32 {
    let tamanho = notas.len();
    let mut soma = 0.0;
    for nota in notas{
        soma += *nota; // se eu tiver armazenando um  valor que vem de uma variavel dde referencia eu tenho que armazenar passando o *
    }

    return soma / tamanho as f32 // convertendo inteiro para float

}

fn main() {

    let notas = [7.5, 8.0, 9.0, 6.5];
    let media = calcula_media(&notas);

    println!("A media das notas é {}", media);

}