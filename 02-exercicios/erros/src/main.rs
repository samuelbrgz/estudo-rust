fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("divisão por zero"))
    } else {
        Ok(a / b)
    }
}
fn main() {
    match dividir(10, 2) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(erro) => println!("Erro: {}", erro),
    }
    match dividir(10, 0) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(erro) => println!("Erro: {}", erro),
    }
}
