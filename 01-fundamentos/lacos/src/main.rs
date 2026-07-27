fn main() {
    // Loop
    let mut n = 0;

    println!("\nQuadrados perfeitos ate 1000: ");
    loop {
        n = n + 1;

        if (n * n) > 1000 {
            break;
        }
        let result = n * n;
        println!("{result}, raiz: {n}")
    }

    //for
    println!("\nMultiplos de 3: ");
    for i in 1..=30 {
        if i % 3 == 0 {
            println!("{}", i);
        }
    }
}
