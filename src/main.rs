mod iteradores;
mod analisador;
use analisador::*;
use std::io::{self, Write};

fn main() {
    loop {
        print!("Digite uma expressão (ou pressione Enter para sair): ");
        io::stdout().flush().unwrap();

        let mut expression = String::new();
        io::stdin().read_line(&mut expression).unwrap();
        let expression = expression.trim();

        if expression.is_empty() {
            println!("Encerrando.");
            break;
        }

        let mut analisador = Analisador::new(expression);

        loop {
            match analisador.proximo() {
                Ok((pos, token)) => {
                    println!("Token: {}, posição: {}", token, pos);
                }
                Err(None) => {
                    println!("Fim da análise");
                    break;
                }
                Err(Some(pos)) => {
                    println!("Erro na posição {}", pos);
                    break;
                }
            }
        }
        println!();
    }
}