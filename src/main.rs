use std::env; // Para ler o argumento do terminal
use std::fs; // para ler o arquivo

fn main(){
    // Coleta os argumentos em um Vetor (uma lista no Heap)
    let args: Vec<String> = env::args().collect();

    // O primeiro argumento (índice 0) é sempre o nome do próprio programa.
    // Precisamos de pelo menos o arquivo (1) e a palavra (2).
    if args.len() < 3 {
        println!("Uso correto: cargo run -- <arquivo> <palavra>");
        return;
    }

    let caminho = &args[1];
    let busca = &args[2];

    println!("Buscando por '{}' em '{}'...\n", busca, caminho);
    match fs::read_to_string(caminho){
        Ok(conteudo)=>{
            let mut total_encontrados = 0;

            for (num_linha, linha) in conteudo.lines().enumerate(){
                if linha.contains(busca){
                    println!("[Linha {}] {}\n", num_linha + 1, linha);
                    total_encontrados += 1;
                }
                
            }
            println!("\nFim da busca. Encontramos {} ocorrências.", total_encontrados);

        }
            Err(e) => println!("Erro ao abrir o arquivo: {}", e),
        
    }
}
