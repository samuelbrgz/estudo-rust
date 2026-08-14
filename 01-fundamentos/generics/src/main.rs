trait ContaCaracteres {
    fn conta_caracteres(&self) -> usize;
}

impl ContaCaracteres for i32 {
    fn conta_caracteres(&self) -> usize {
        self.to_string().chars().count()
    }
}
impl ContaCaracteres for f32 {
    fn conta_caracteres(&self) -> usize {
        self.to_string().chars().count()
    }
}
impl ContaCaracteres for String {
    fn conta_caracteres(&self) -> usize {
        self.chars().count()
    }
}
fn qntd_caracteres<T: ContaCaracteres>(valor: T) -> usize {
    valor.conta_caracteres()
}
fn contar_posições<T>(vetor: &[T]) -> usize {
    vetor.len()
}
fn main() {
    let vetorint = [1, 2, 3, 4, 5, 6];
    let vetorfloat = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let vetorstrings = ["oi", "to"];

    let x = 12345;
    let y = 123.45;
    let z = String::from("Ola mundo");

    println!("tamanho vetor de inteiros: {}", contar_posições(&vetorint));
    println!("tamanho vetor de floats: {}", contar_posições(&vetorfloat));
    println!(
        "tamanho vetor de strings: {}",
        contar_posições(&vetorstrings)
    );

    println!("Tamanho x: {}", qntd_caracteres(x));
    println!("Tamanho y: {}", qntd_caracteres(y));
    println!("Tamanho z: {}", qntd_caracteres(z));
}
