fn main() {
    let nota = 75;

    if nota >= 90 {
        println!("A");
    } else if nota >= 80 {
        println!("B");
    } else if nota >= 70 {
        println!("C");
    } else {
        println!("Reprovado");
    }

    let sim: bool = nota >= 75;

    println!("Foi aprovado? {sim}");
}
