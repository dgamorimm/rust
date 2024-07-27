struct User{
    username: String,
    email: String,
    ativo: bool,
    genero : String
}

impl User {
    fn nome_do_usuario(&self){
        println!("O nome do usuario é: {}", self.username);
    }

    fn ativo_ou_inativo(&self){
        println!("O usuario esta ativo ? {}", self.ativo);
    }
}

fn main() {

    let mut pessoa = User {
        username: String::from("Douglas"),
        email: String::from("douglas@dominio.com"),
        ativo: true,
        genero: String::from("masculino")
    };

    pessoa.nome_do_usuario();
    pessoa.ativo_ou_inativo();
}