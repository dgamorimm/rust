use std::fs::File;  //importar/criar o arquivo
use std::io::prelude::*;  // acessar/escrever o conteúdo do arquivo

fn main() {

    let mut arquivo = File::create("src/working_files/rust_write_file.txt").expect("Falha ao criar o arquivo");
    
    arquivo.write_all(b"Arquivo de teste sendo criado").expect("Falha ao escrever no arquivo");
}