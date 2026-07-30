use std::io;

fn fibonacci(x: u128) {
    let mut n: u128 = 1;
    let mut n_ant: u128 = 0;

    for _ in 0..x {
        print!("{n}, ");
        let temp = n + n_ant;
        n_ant = n;
        n = temp;
    }
}

fn main() {
    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada).expect("Falha ao ler.");

    let numero_caracteres: u128 = entrada.trim().parse().expect("Numero invalido");

    fibonacci(numero_caracteres);
}
