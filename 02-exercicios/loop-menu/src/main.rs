use std::io;

fn main() {
    loop {
        println!("escolha uma opcao: ");
        println!("opcao 1\nopcao 2\nopcao 3\nSair 4");

        let mut entrada = String::new();

        io::stdin().read_line(&mut entrada).expect("Falha ao ler");

        let opcao: i8 = entrada.trim().parse().expect("Opcao invalida");

        match opcao {
            1 => println!("Voce escolheu a opção 1"),
            2 => println!("Voce escolheu a opção 2"),
            3 => println!("Voce escolheu a opção 3"),
            4 => {
                println!("Voce escolheu sair");
                break;
            }
            _ => println!("Opção invalida"),
        }
    }
}
