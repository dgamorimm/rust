struct Pessoa{
    nome: String,
    idade: i32
}

trait Voz {
    fn falar(&self);
    fn tem_voz(&self) -> bool;
}

impl Voz for Pessoa {
    fn falar(&self) {
        println!("Olá meu nome é {}", self.nome);
    }

    fn tem_voz(&self) -> bool {
        if self.idade > 0 {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {

    let pessoa = Pessoa{
        nome: String::from("Douglas"),
        idade: 20
    };

    pessoa.falar();
    println!("Tem voz ? {}", pessoa.tem_voz());
}