use std::fs; // Biblioteca para lidar com o sistema de arquivos

fn main(){
    let caminho = "exemplo.txt";
    println!("Tentando ler o arquivo: {}", caminho);

    // fs::read_to_string retorna um 'Result'. 
    // Usamos o 'match' para tratar os dois caminhos possíveis.
    match fs::read_to_string(caminho){
        Ok(conteudo)=>{
            println!("--- Conteúdo do Arquivo ---");
            for linha in conteudo.lines(){
                if linha.contains("Soldados"){
                println!("{}", linha);
            }
            }
        }
        Err(erro) =>{
            println!("Erro ao ler o arquivo: {}", erro);
            println!("Dica de Suporte: Verifique se o arquivo existe ou se você tem permissão para acessá-lo.");
        }
    }
}