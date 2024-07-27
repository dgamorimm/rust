struct User{
    username: String,
    email: String,
    ativo: bool,
    genero : String
}

fn main() {

    let mut pessoa = User{
        username: String::from("Douglas"),  // slice string é utilizado para o tipo struct
        email: String::from("douglas@dominio.com"),
        ativo: true,
        genero: String::from("masculino")
    };

    pessoa.ativo = false;
    println!("pessoa: {}\nemail: {}\nativo: {}\ngenero: {}", pessoa.username, pessoa.email, pessoa.ativo, pessoa.genero);

}