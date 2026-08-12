mod matematica;
use matematica::{dividir, multiplicar, soma, subtrair};

fn main() {
    let x: f64 = 56.0;
    let y: f64 = 48.0;

    println!("soma: {}", soma(x, y));
    println!("subtraçao: {}", subtrair(x, y));
    println!("multiplicaçao: {}", multiplicar(x, y));
    println!("divisao: {}", dividir(x, y));
}
