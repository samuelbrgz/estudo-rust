trait Notificador {
    fn enviar(&self, mensagem: &str) -> String;
}

struct Email {
    endereco: String,
}
struct SMS {
    numero: String,
}
struct Push {
    dispositivo: String,
}

impl Notificador for Email {
    fn enviar(&self, mensagem: &str) -> String {
        format!("Email para {}: {}", self.endereco, mensagem)
    }
}
impl Notificador for SMS {
    fn enviar(&self, mensagem: &str) -> String {
        format!("SMS para {}: {}", self.numero, mensagem)
    }
}
impl Notificador for Push {
    fn enviar(&self, mensagem: &str) -> String {
        format!("Push para {}: {}", self.dispositivo, mensagem)
    }
}

fn criar_notificador(tipo: &str, destino: String) -> Box<dyn Notificador> {
    if tipo == "email" {
        Box::new(Email { endereco: destino })
    } else if tipo == "sms" {
        Box::new(SMS { numero: destino })
    } else if tipo == "push" {
        Box::new(Push {
            dispositivo: destino,
        })
    } else {
        panic!("Tipo de notificador desconhecido!");
    }
}
fn enviar_todos(notificadores: &[Box<dyn Notificador>], mensagem: &str) {
    for c in notificadores {
        let msg = c.enviar(mensagem);
        println!("{}", msg);
    }
}
fn main() {
    let x: Vec<Box<dyn Notificador>> = vec![
        criar_notificador("email", "samuelvborgess@gmail.com".to_string()),
        criar_notificador("sms", "+55659999404046".to_string()),
        criar_notificador("push", "A56_samuel".to_string()),
    ];
    enviar_todos(&x, "Cadastro realizado com sucesso");
}
