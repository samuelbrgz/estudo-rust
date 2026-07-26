fn main() {
    let idade: u8 = 20;
    let temperatura: i32 = -32;
    let distancia: u64 = 298000;

    println!("idade: {idade} anos, temperatura: {temperatura} C, distancia: {distancia} km");
    println!("u8 vai de {}, até {}", u8::MIN, u8::MAX);
    println!("u16 vai de {}, até {}", u16::MIN, u16::MAX);
    println!("u32 vai de {}, até {}", u32::MIN, u32::MAX);
    println!("u64 vai de {}, até {}", u64::MIN, u64::MAX);
    println!("\n");

    let pi: f64 = 3.14159;
    let aprovado: bool = true;
    let letra: char = 'S';

    println!("Pi: {pi}, aprovado: {aprovado}, Inicial: {letra}");
    println!("\n");

    let a: u32 = 50;
    let b: u64 = 50;
    let soma = a as u64 + b;
    println!("Valor de a: {a}, Valor de b: {b}, Soma: {soma}")
}
