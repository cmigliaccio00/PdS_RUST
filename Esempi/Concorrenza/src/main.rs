use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    /*-----------------Soluzione con Mutex + Condvar
    let pair = Arc::new(
        (
            Mutex::new(Vec::<i32>::new()),
            Condvar::new()
        )
    );

    let pair2 = pair.clone();
    //equivalente a std::clone(&pair);

    //Da qui creo un altro flusso di esecuzione (thread)
    let t =thread::spawn(move||{
        let (m,cv) = &*pair2;
        for i in 0..10{
            thread::sleep(Duration::from_nanos(1));
            //Prendo possesso del Mutex
            let mut v= m.lock().unwrap();

            //Popolo il vettore (dopo aver acquisito il lock)
            v.push(i);
            //Di solito (se non so cosa usare), notify_all()
            //va bene sempre --> meno efficiente, ma funziona
            cv.notify_all();
        }
    });

    //Altra attività qui o in un thread secondario...
    let mut round=0;
    while round!=10{
        let  (m,cv) = &*pair;
        let mut v = m.lock().unwrap();

        //Finche'  ci sono 0 valori non ho nulla da fare;
        while round==v.len(){
            v = cv.wait(v).unwrap();
        }

        println!("While sleeping {} values produced", v.len()-round);
        for i in round..v.len(){
            println!("{}", v[i]);
        }
        round=v.len();
    }
    //Dopo questo viene rilasciato il lock e sveglio l'altro processo

    //Lo faccio solo per pulizia del codice
    t.join().unwrap();

    //Se tolgo la sleep, cosa succede?
    //Non riesco ad alternare i due thread (principale e secondario)
     */

    //2 semantica: Condivisione di messaggi
    let (tx, rx) = channel();

    let t1 = thread::spawn(move||{
       for i in 0..10{
           //thread::sleep(Duration::from_secs(3));
           //Metto nel canale
           tx.send(i).unwrap();
       }
        thread::sleep(Duration::from_secs(3));
    });

    //Prelevo dal canale
    while let Ok(i) = rx.recv(){
        println!("Received {}", i);
    }

    //Finché non ho fatto l'ultimo drop, aspetto.
    println!("fine!");

    //Aspetto la terminazione del thread
    t1.join().unwrap();
}
