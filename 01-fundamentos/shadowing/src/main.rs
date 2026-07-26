fn main() {
    let x = 5;
    println!("Valor de X: {}, seu endereço de memoria: {:p}", x, &x);

    let x = x + 1;
    println!("Valor de X: {}, seu endereço de memoria: {:p}", x, &x);

    let x = x * 2;
    println!("Valor de X: {}, seu endereço de memoria: {:p}", x, &x);
}
