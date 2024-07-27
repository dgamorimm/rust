
fn main() {

    let numero = 8;

    match numero {
        1=> println!("O numero é 1"),
        2=> println!("O numero é 2"),
        3=> println!("O numero é 3"),
        4 | 5 | 6 => println!("O numero é 4, 5 ou 6"),
        7..=9 => println!("O numero é 7, 8 ou 9"),
        _=> println!("O numero não é 1, 2 ou 3")
    }
}