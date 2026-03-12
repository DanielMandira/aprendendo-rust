use std::thread;
use std::time::Duration;

fn main(){
    // Criamos uma nova thread (uma "Tarefa Paralela")
    let tarefa_1 = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread A: Analisando Log de Rede... {}", i);
            thread::sleep(Duration::from_millis(5000)); // Simulando Carga
        }
    });

    let tarefa_2 = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread B: Verificando Log de Segurança... {}", i);
            thread::sleep(Duration::from_millis(7000)); // Simulando Carga
        }
    });

    tarefa_1.join().unwrap(); // Espera a tarefa 1 terminar
    tarefa_2.join().unwrap(); // Espera a tarefa 2 terminar

    println!("Todos os logs foram processados simultaneamente!!");
}
