

fn main() { //bloco1
    let a = 10;

    {//bloco2
        println!("O valor de a : {}", a);
        let a = 20.309; // shadowing
        println!("O valor de a : {}", a); // shadowing
    }//bloco2

    println!("O valor de a : {}", a);
}//bloco1