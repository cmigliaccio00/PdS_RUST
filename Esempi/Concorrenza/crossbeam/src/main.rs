//Modello di concorrenza: FAN-IN / FAN-OUT
use std::thread;
use std::time::Duration;
use crossbeam;
use crossbeam::channel::{bounded, Sender, Receiver};
use rand::random;

fn worker(adder: i32, tx: Sender<i32>, rx:Receiver<i32>){
    while let Ok(intero) = rx.recv(){
        tx.send(intero+adder).unwrap();
        //thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    //Primo canale di comunicazione
    let (tx_input, rx_input)= bounded::<i32>(10);
    let (tx_output, rx_output) = bounded::<i32>(10);
    let mut workers = Vec::new();

    //Primo step: Produco tutti 0
    let prod = thread::spawn(move||{
        //Produco i valori che servono
        for _ in 0..10{
            tx_input.send(0).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //Step intermedio: chiamo dei worker per farmi incrementare i valori
    for i in 1..5{
        let tx_cp = tx_output.clone();
        let rx_cp=rx_input.clone();
        workers.push(thread::spawn(
            move|| worker(i,tx_cp, rx_cp)
        )
        );
    }

    //Stage finale: Consumatore
    let cons =  thread::spawn(move||{
        while let Ok(ris) = rx_output.recv(){
            thread::sleep(Duration::from_secs(1));
            println!("Risultato finale: {}", ris);
        }
    });

    drop(tx_output);

    //Attesa dei thread
    prod.join().unwrap();
    cons.join().unwrap();
    for handle in workers{
        handle.join().unwrap();
    }
}
