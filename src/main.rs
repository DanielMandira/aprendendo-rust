// fn main() {
//     //1. STACK (TAMANHO FIXO, RÁPIDO)
//     let idade: i32 = 28; // UM NÚMERO INTEIRO DE 32 BITS VAI DIRETO PARA O STACK.
//     let ativo: bool =  true; // UM VALOR BOOLEANO TAMBÉM VAI PARA O STACK.

//     //2. HEAP (TAMANHO VARIÁVEL, MAIS LENTO)
//     // Usamos String::from para criar um texto que pode crescer.
//     // O texto "Analisata de Suporte" fica no HEAP.
//     // O "ponteiro" (a chave do quarto) fica no STACK.
//     let cargo = String::from("Analista de Suporte Junior");

//     println!("Idade: {}, Ativo: {}, Cargo: {}", idade, ativo, cargo);

// } // <--- Aqui o Rust limpa a memória automaticamente!


// fn main(){
//     let s1 = String::from("Suporte");
//     let s2 = s1; // s1 é movida para s2, e s1 não é mais válida.)
//     println!("{}", s1);
// }


// fn main(){
//     let cargo = String::from("Analista de Suporte Junior");
    
//     // Passamos o cargo com '&'. estamos dizendo: "Pode ler mas ainda é meu"
//     exibir_cargo(&cargo);
//     println!("Ainda sou o dono do Texto: {}", cargo)
// }

// // A função recebe '&String', (uma referencia), não a 'String' em si. Isso é chamado de "borrowing" (emprestimo).
// fn exibir_cargo(texto: &String){
//     println!("Processando o cargo: {}", texto);

// } // O emprestimo termina aqui, mas o cargo original ainda é válido no main. O dado no HEAP não é apagado.


fn main(){
    let mut cargo = String::from("Junior");

    // Passamos o cargo com '&mut'. estamos dizendo: "Pode ler e também pode mudar, mas ainda é meu"
    atualizar_cargo(&mut cargo);
    println!("Cargo atualizado: {}", cargo);

}

// A função recebe '&mut String', (uma referencia mutável), não a 'String' em si. Isso é chamado de "borrowing" (emprestimo).
fn atualizar_cargo(texto: &mut String){
    texto.push_str(" Senior")
}