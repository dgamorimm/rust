

fn main() { //bloco1
    let a = 10;

    {//bloco2
        let b =15;
        println!("A soma de {} e {} é {}", a, b, a + b);
    }//bloco2

    println!("A soma de {} e {} é {}", a, b, a + b); // não consegue acessar o bloco de código que tem a variavel b
}//bloco1