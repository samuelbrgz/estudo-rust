struct Conta {
    titular: String,
    saldo: i32,
    tipo: String,
}

fn exibir(conta: &Conta) {
    println!(
        "titular: {}\nsaldo: {}\ntipo de conta: {}",
        conta.titular, conta.saldo, conta.tipo
    );
}

fn sacar(conta: &mut Conta, x: i32) {
    if x > conta.saldo {
        println!("Saldo insuficiente!");
    } else {
        conta.saldo = conta.saldo - x;
        println!("Saque realizado no valor de: {}", x);
        exibir(&conta);
    }
}

fn main() {
    let mut conta1 = Conta {
        titular: String::from("Samuel"),
        saldo: 5000,
        tipo: String::from("corrente"),
    };
    let x = 2000;
    let y = 4000;

    exibir(&conta1);
    sacar(&mut conta1, x);
    sacar(&mut conta1, y);
}
