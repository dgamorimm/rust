fn is_permutation(str1: &str, str2: &str) -> bool {
    
    if str1.len() != str2.len() {
        return false // Diferentes comprimentos, não podem ser permutações
    }

    let mut contagem_caracteres = [0; 128]; // Cria um array de 128 posicoes. Assumindo caracteres ASCII

    // conta as ocorrencias de caracteres da primeira string
    for &c in str1.as_bytes() {
        contagem_caracteres[c as usize] += 1;
    }

    // Decrementa as ocorrencias de caracteres da segunda string
    for &c in str2.as_bytes() {
        contagem_caracteres[c as usize] -= 1;
        if contagem_caracteres[c as usize] < 0 {
            return false // mais ocorrencias do caracter na segunda string
        }
    }
    
    return true // Todos os caracteres da primeira string tem ocorrencias correspondentes na segunda string
}

fn main() {

    let str1 = "roma";
    let str2 = "amor";

    if is_permutation(str1, str2){
        println!("{} e {} são permutações", str1, str2);
    } else {
        println!("{} e {} não são permutacoes", str1, str2);
    }
}