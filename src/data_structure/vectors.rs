fn main() {

    let mut vetores = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let mut vetores2: Vec<i32> = Vec::new(); // esse é um método que ja reserva uma espaço na memória para usar esse vetor

    println!("O primeiro elemento do vetores é {}", vetores[0]);

    vetores.push(11);
    println!("O ultimo valor do vetor é {}", vetores[vetores.len() - 1]);

    vetores.remove(1);
    println!("Todos os valores : {:?}", vetores);

    for i in vetores.iter(){
        println!("{}", i);
    }
}