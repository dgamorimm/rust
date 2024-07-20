#[derive(Debug)]
enum Pagamento{
    Dinheiro(f32),
    CreditoCartao(bool, f32),
    DebitoCartao(bool, f32)
}

fn main() {

    let pessoa_pagamento = Pagamento::CreditoCartao(false,100f32);
    match pessoa_pagamento{
        Pagamento::Dinheiro(f) => println!("Pessoa pagou com dinheiro {}!", f),
        Pagamento::CreditoCartao(true, x) => println!("Pessoa pagou com cartão de credito {}!", x),
        Pagamento::CreditoCartao(false, _) => println!("O pagamento do cartão de crédito não funcionou!"),
        _ => {} // não é recomendavel pela linguam, o certo é colocar todos os matchs
    }
}