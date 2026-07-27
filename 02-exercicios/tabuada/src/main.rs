use std::io;

fn main() {
    // let mut n = 0;
    //
    // println!("\nTabuada de 0 a 10: ");
    // while n <= 10 {
    //
    //     println!("\nTabuada do {n}:");
    //
    //     for i in 0..=10 {
    //
    //         let resultado = n * i;
    //         println!("{n} x {i} = {resultado}");
    //
    //     }
    //     n = n + 1;
    // }
    let mut entrada = String::new();
    println!("Qual numero deseja consultar a tabuada?");

    io::stdin().read_line(&mut entrada).expect("Falha ao ler");

    let numero: i16 = entrada.trim().parse().expect("Não é um numero");

    println!("Tabuada do {numero}");

    for i in 0..=10 {
        let resultado = numero * i;
        println!("{numero} x {i} = {resultado}");
    }
}
