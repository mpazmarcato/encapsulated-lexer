mod iteradores;
mod analisador;
use analisador::Analisador;
use std::io::{self, Write};

fn main() {
    loop {
        print!("Digite uma expressão (ou pressione Enter para sair): ");
        io::stdout().flush().unwrap();

        let mut expressão = String::new();
        io::stdin().read_line(&mut expressão).unwrap();

        let expressão = expressão.trim();
        if expressão.is_empty() {
            println!("Encerrando.");
            break;
        }
        println!("\nAnalisando: '{}'", expressão);

        let mut analisador = Analisador::novo(expressão);
        let mut resultado = Vec::new();
        loop {
            match analisador.próximo() {
                Ok((pos, token)) => {
                    resultado.push(format!("(\"{}\", {})", token, pos));
                }
                Err(None) => {
                    break;
                }
                Err(Some(pos)) => {
                    resultado.push(format!("Erro na posição {}", pos));
                    break;
                }
            }
        }

        if !resultado.is_empty() {
            let output = resultado.join(" ");
            println!("{}", output);
        } else {
            println!("Nenhum token encontrado.");
        }
        println!();
    }
}