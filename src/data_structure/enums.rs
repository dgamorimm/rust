enum Direction{
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)] // isso faz com que conseguimos imprimir na tela
enum Gender{
    Male,
    Female
}

fn main() {

    let player:Direction = Direction::Right;
    let player_male:Gender = Gender::Male;

    match player {
        Direction::Up => println!("O jogador foi para cima"),
        Direction::Down => println!("O jogador foi para baixo"),
        Direction::Left => println!("O jogador foi para esquerda"),
        Direction::Right => println!("O jogador foi para direita"),
    }

    println!("O jogador é do genero {:?}", player_male);
}