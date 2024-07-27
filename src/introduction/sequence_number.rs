
fn main() {

    println!("Por favor, digite uma sequencia de números reais:");

    let mut input = String::new();

    // Read sequence user numbers

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<f64> = input
        .trim()  // remover os espacos
        .split_whitespace()  // far um split por espacos
        .map(|x| x.parse::<f64>().expect("Por favor, insira números reais")) // parse para float
        .collect(); // transforma em vector de floats

    let mut sum: f64 = 0.0;

    for num in &numbers {  // & faz referencia aos valores reais, ele não precisa fazer uma cópia para o valor passado. Isso otimiza muito a memória pois ele só tema  capacidade de leitura.
        
        if num % 2.0 == 0.0 {
            sum += num;
        }
        
    }

    println!("A soma dos numeros pares: {}", sum);
}