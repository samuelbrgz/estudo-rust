struct Usuario {
    nome: String,
    idade: u32,
    ativo: bool,
}

fn main() {
    let user1 = Usuario {
        nome: String::from("Samuel"),
        idade: 20,
        ativo: true,
    };
    println!("Nome: {}", user1.nome);
    println!("Idade: {}", user1.idade);
    println!("Ativo: {}", user1.ativo);
}
