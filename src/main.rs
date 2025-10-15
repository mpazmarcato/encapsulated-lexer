mod iteradores;
mod analisador;
use iteradores::*;
use analisador::*;

fn main() {
    let exemplos = [
        "450 + 20",
        "450     +     20",
        "450+20",
        "0+-0",
        "0 +++",
        "10+a",
        "10 + 20a",
    ];

    for entrada in exemplos {
        println!("{}", entrada);
        let mut analisador = Analisador::new(entrada);

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