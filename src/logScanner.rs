use std::env; // Para ler o argumento do terminal
use std::fs; // para ler o arquivo
use colored::*;


fn main(){
    // Coleta os argumentos em um Vetor (uma lista no Heap)
    let args: Vec<String> = env::args().collect();

    // O primeiro argumento (índice 0) é sempre o nome do próprio programa.
    // Precisamos de pelo menos o arquivo (1) e a palavra (2).
    if args.len() < 3 {
        println!("{}", "Uso: scanner <arquivo> <palavra>".yellow());
        return;
    }

    let caminho = &args[1];
    let busca = &args[2];

    println!("Buscando por '{}' em '{}'...\n", busca, caminho);
    match fs::read_to_string(caminho){
        Ok(conteudo)=>{
            for (i, linha) in conteudo.lines().enumerate(){
                if linha.contains(busca){
                    println!("{}: {}", (i + 1).to_string().blue(), linha.on_red().white());
                }
                
            }
        }
            Err(e) => println!("{} {}", "Erro crítico:".red().bold(), e),
        
    }
}
