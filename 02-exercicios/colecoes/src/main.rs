trait Colecao {
    fn imprimir(&self);
    fn adicionar(&mut self, valor: i32);
    fn remover(&mut self) -> Option<i32>;
}

struct Pilha {
    itens: Vec<i32>,
}
struct Fila {
    itens: Vec<i32>,
}

impl Colecao for Pilha {
    fn imprimir(&self) {
        println!("pilha:");
        for x in &self.itens {
            println!("{}", x);
        }
    }
    fn adicionar(&mut self, valor: i32) {
        self.itens.push(valor);
        self.imprimir();
    }
    fn remover(&mut self) -> Option<i32> {
        let resultado = self.itens.pop();
        self.imprimir();
        resultado
    }
}
impl Colecao for Fila {
    fn imprimir(&self) {
        println!("fila:");
        for x in &self.itens {
            println!("{}", x);
        }
    }
    fn adicionar(&mut self, valor: i32) {
        self.itens.push(valor);
        self.imprimir();
    }
    fn remover(&mut self) -> Option<i32> {
        let resultado = if self.itens.is_empty() {
            None
        } else {
            Some(self.itens.remove(0))
        };
        self.imprimir();
        resultado
    }
}

fn main() {
    let mut fila = Fila { itens: Vec::new() };
    let mut pilha = Pilha { itens: Vec::new() };

    pilha.adicionar(20);
    pilha.adicionar(30);
    pilha.adicionar(40);
    pilha.adicionar(50);
    pilha.remover();
    pilha.remover();
    pilha.remover();
    pilha.remover();

    fila.adicionar(20);
    fila.adicionar(30);
    fila.adicionar(40);
    fila.adicionar(50);
    fila.remover();
    fila.remover();
    fila.remover();
    fila.remover();
}
