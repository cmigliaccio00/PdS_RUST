use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main(){
    let mut my_box = Box::new(4);
    let mut my_vec = Vec::new();
    my_vec.push(1);

    let a = Arc::new(Mutex::new(0));
    let a1 = a.clone();
    let a2 = a.clone();

    let t1_handle = thread::spawn(move ||{
        *my_box = 5;
        println!("Primo thread, consumato il box");
        //cthread::sleep(Duration::from_nanos(3));
        for _ in 1..11{
            let mut v = a2.lock().unwrap();
            *v += 1;
        }
        return 0;
    });

    let t2_handle = thread::spawn(move||{
        for el in my_vec{
            println!("Elemento {}", el);
        }
        println!("Fine secondo thread");
        for _ in 1..11{
            let mut v = a1.lock().unwrap();
            *v +=1;
        }
        return 0;
    });

    println!("Valore del mutex qui: {}", *a.lock().unwrap());
    //Attendo la fine dei due thread
    println!("Primo thread terminato! Ris: {}", t1_handle.join().unwrap());
    println!("Secondo thread terminato! Ris: {}", t2_handle.join().unwrap());
    println!("Valore finale della variabile: {}", *a.lock().unwrap());
}