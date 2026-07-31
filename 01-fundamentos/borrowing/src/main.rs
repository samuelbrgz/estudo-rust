// fn tamanho(s: &String) -> usize {
//     s.len()
// }
// fn main() {
//     let s1 = String::from("Teste");
//     let x = tamanho(&s1);
//
//     println!("{}, tamanho {}", s1, x);
// }
// fn main() {
//     let mut s = String::from("teste");
//
//     let r1 = &mut s;
//     let r2 = &mut s;
//
//     println!("{}", r1);
//     println!("{}", r2);
// }

fn mudar(x: &mut String) {
    x.push_str(" mundo")
}

fn main() {
    let mut s1 = String::from("ola,");
    mudar(&mut s1);

    println!("{}", s1);
}
