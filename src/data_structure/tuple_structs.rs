struct User (String, String, bool, String); // strict de tuplas ou tuple structs

// fn user(usuario : &User){

//     println!("O nome do usuario é: {}", usuario.username);
// }

fn main() {

    let mut pessoa = User(
        String::from("Douglas"), 
        String::from("douglas@dominio.com"), 
        true, String::from("masculino")
    );
    
    pessoa.0 = String::from("Douglas Amorim");

    println!("pessoa: {}\nemail: {}\nativo: {}\ngenero: {}", pessoa.0, pessoa.1, pessoa.2, pessoa.3);

}