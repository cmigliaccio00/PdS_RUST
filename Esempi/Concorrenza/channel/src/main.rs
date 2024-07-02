use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::sync::mpsc::{channel, sync_channel};
use std::time::Duration;

fn main() {
    let (tx, rx) = sync_channel(3);
    let txp = tx.clone();

    let producer = thread::spawn(move||{
        for i in 1..10{
            txp.send(1).unwrap();
            println!("Produco {}", i);
        }
    });

    drop(tx);

    let consumer = thread::spawn(move||{
        while let Ok(ris) = rx.recv(){
                        println!("Valore {}", ris);
            thread::sleep(Duration::from_secs(2));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
