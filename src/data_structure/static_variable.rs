static mut STATIC_VARIABLE: i32 = 15; // as variaveis estáticas são variaveis a nivel global

fn main() { //bloco1

    unsafe{//bloco2
        println!("O valor da STATIC_VARIABLE : {}", STATIC_VARIABLE);
    }//bloco2

    //println!("O valor da STATIC_VARIABLE : {}", STATIC_VARIABLE); // não consegue acessar o bloco de código que tem a variavel STATIC_VARIABLE, tem que passar dentro de um outro bloco com unsafe. Não é recomendavel
}//bloco1