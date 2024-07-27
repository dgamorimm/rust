
use std::collections::HashMap;

fn main() {

    let mut hash_map = HashMap::new();

    hash_map.insert("Matemática", 90);
    hash_map.insert("Português", 72);
    hash_map.insert("Biologia", 58);
    hash_map.insert("Informática", 96);

    println!("Quantas matérias o estudante tem ? {}", hash_map.len());

    match hash_map.get("Informática"){
        Some(grade) => println!("A sua nota na matemática é {}", grade),
        None => println!("A matemática não foi avaliada")
        
    }

    hash_map.remove("Português");
    println!("O aluno estuda Português? {}", hash_map.contains_key("Português"));
}