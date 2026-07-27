use std::io;

fn conversao(x: i8) {
    if x == 1 {
        println!("Qual a temperatura em celsius?");

        let mut entrada = String::new();

        io::stdin().read_line(&mut entrada).expect("Falha ao ler");

        let temp_ce: f32 = entrada.trim().parse().expect("Numero invalido");
        let temp_fa: f32 = (temp_ce * 1.8) + 32.0;

        println!("temperatura em Fahrenheit: {} F", temp_fa);
    } else {
        println!("Qual a temperatura em Fahrenheit?");

        let mut entrada = String::new();

        io::stdin().read_line(&mut entrada).expect("Falha ao ler");

        let temp_fa: f32 = entrada.trim().parse().expect("Numero invalido");
        let temp_ce: f32 = (temp_fa - 32.0) / 1.8;

        println!("temperatura em celsius: {} C", temp_ce);
    }
}

fn main() {
    loop {
        println!("====== Conversor de Temperatura ======");
        println!(
            "Voce deseja converter de:\n 1-Celsius para Fahrenheit\n 2-Fahrenheit para Celsius\n3-Sair "
        );
        let mut entrada = String::new();

        io::stdin().read_line(&mut entrada).expect("Falha ao ler");

        let opcao: u8 = entrada.trim().parse().expect("Numero invalido");

        match opcao {
            1 => conversao(1),
            2 => conversao(2),
            3 => {
                println!("saindo...");
                break;
            }
            _ => println!("Opção invalida"),
        }
    }
}
