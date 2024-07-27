#[derive(Debug)]
enum Pagamento{
    Dinheiro,
    CreditoCartao,
    DebitoCartao
}

fn main() {

    let pessoa_pagamento = Pagamento::CreditoCartao;
    match pessoa_pagamento{
        Pagamento::Dinheiro => println!("Pessoa pagou com dinheiro!"),
        Pagamento::CreditoCartao => println!("Pessoa pagou com cartão de credito!"),
        _ => {} // não é recomendavel pela linguam, o certo é colocar todos os matchs
    }
}