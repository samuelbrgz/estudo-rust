struct CONTABANCARIA {
    titular: String,
    saldo: i32,
    limite: i32,
}

fn sacar(conta1: &mut CONTABANCARIA, valor: i32) {
    let total: i32 = conta1.saldo + conta1.limite;

    if total >= valor {
        if valor <= conta1.saldo {
            conta1.saldo = conta1.saldo - valor;
            println!("Titular: {}", conta1.titular);
            println!("Saldo atualizado: {}", conta1.saldo);
        } else if valor > conta1.saldo {
            conta1.limite = conta1.limite + (conta1.saldo - valor);
            conta1.saldo = 0;
            println!("Titular: {}", conta1.titular);
            println!("Saldo atualizado: {}", conta1.saldo);
            println!("Limite de cheque especial atualizado: {}", conta1.limite);
        }
    } else {
        println!("Titular: {}", conta1.titular);
        println!("Saldo insuficiente!");
    }
}

fn main() {
    let mut conta1: CONTABANCARIA = CONTABANCARIA {
        titular: String::from("Samuel"),
        saldo: 1000,
        limite: 500,
    };
    sacar(&mut conta1, 200);
    sacar(&mut conta1, 300);
    sacar(&mut conta1, 2000);
}
