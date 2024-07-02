use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct MioTipo{
    mio_vec: Vec<i32>,
    counter: i32
}

impl MioTipo{
    fn new()->Self{
        MioTipo{mio_vec: Vec::<i32>::new(), counter:0}
    }
}

//Uso delle condvar: ho un thread che legge un vettore
//quando è stato riempito
fn main() {
    let pair = Arc::new((
            Mutex::new(MioTipo::new()),
            Condvar::new()
        ));

    let pair2 = Arc::clone(&pair);
    let pair3 = Arc::clone(&pair);

    //Thread secondario: consumatore
    let t2_handle = thread::spawn(move||{
        //Destrutturazione della tupla
        let (m, cv) = &*pair2;
        let mut v = m.lock().unwrap();
        v = cv.wait_while(v, |var| var.counter==0).unwrap();

        //Esco dall'attesa e stampo a video il vettore: consumo il vettore
        println!("Mentre stavo aspettando sono stati inseriti: ");
        for i in 0..v.mio_vec.len() {
            println!("Valore {}", v.mio_vec[i]);
        }
    });

    //Thread secondario: produttore
    let t3_handle = thread::spawn(move||{
        let (m, cv) = &*pair3;
        //Accesso in mutua esclusione
        let mut v = m.lock().unwrap();
        for i in 1..10{
            v.mio_vec.push(i);
        }
        println!("Prodotti i valori (thread secondario)");
        cv.notify_all();
    });

    //Thread principale: produttore
    //Se non volessi mettere queste parentesi graffe, dovrei fare il drop
    //esplicito del MutexGuard altrimenti il mutex (nativo) non verrebbe rilasciato più
    {
        let (m,cv) = &*pair;

        let mut v = m.lock().unwrap();

        for i in 11..20{
            v.mio_vec.push(i);
        }
        println!("Prodotti i valori (thread principale)");
        v.counter=1;
        cv.notify_all();
    }

    t2_handle.join().unwrap();
    t3_handle.join().unwrap();
}
