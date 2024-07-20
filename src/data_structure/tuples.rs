

fn main() {

    let tupla = (12, "valores", 3.14, (1, 2, 3));
    println!("{}", (tupla.3).2);

    let (a, b, c, d) = tupla;

    println!("O valor de a : {}", a);
    println!("O valor de b : {}", b);
    println!("O valor de c : {}", c);

    println!("O valor de d : {:?}", d);
}