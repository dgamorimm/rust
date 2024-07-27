fn main() {

    let x = 10;
    let y = &x; // emprestando o valor de x para y
    let z = &x; // todas as refrencias são imutaveis.
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    let mut x2: i32 = 10;
    let w = &mut x2; //Para eu poder alterar uma referencia eu tenho que passar um parameto
    
    *w +=1;
    println!("w: {}", w);
}