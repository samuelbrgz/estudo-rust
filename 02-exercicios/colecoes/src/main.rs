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
fn testar(colecao: &mut impl Colecao) {
    colecao.adicionar(20);
    colecao.adicionar(30);
    colecao.adicionar(40);
    colecao.adicionar(50);
    colecao.remover();
    colecao.remover();
    colecao.remover();
    colecao.remover();
}

fn main() {
    let mut fila = Fila { itens: Vec::new() };
    let mut pilha = Pilha { itens: Vec::new() };

    testar(&mut fila);
    testar(&mut pilha);
}
