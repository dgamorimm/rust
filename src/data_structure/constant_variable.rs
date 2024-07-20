
const NUMERO_PI : f32 = 3.14159; // variavel constante ou imutavel

fn comprimento_circuferencia(r: f32) -> f32 {
    let mut c = 2f32 * NUMERO_PI * r;
    return c
}

fn main() {
    println!("O comprimento da circunferência e: {}", comprimento_circuferencia(10.0));
}