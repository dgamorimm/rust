
fn main() {

    let numeros_inteiros: [i32; 5] = [1, 2, 3, 4, 5]; // array de inteiros
    let numeros_dois_repetidos: [i32; 1000] = [2;1000]; // array de 1000 elementos com o valor 2
    println!("Numero: {}", numeros_inteiros[4]);

    for i in 0..numeros_inteiros.len(){
        println!("Com range: {}", numeros_inteiros[i]);
    }

    println!("\n");

    for n in numeros_dois_repetidos.iter(){
        println!("Com iterador: {}", n);
    }
}