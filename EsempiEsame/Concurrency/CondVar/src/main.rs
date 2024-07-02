use std::sync::{Condvar, Mutex};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
//Vorrei che il secondo thread partisse solo dopo che il primo thread
//ha stampato un messaggio

//Questo è il tipo a cui faccio riferimento
struct attendi{
    intero: i32,
    flag: bool
}

impl attendi{
    fn new()->Self{
        Self{intero:0, flag:false}
    }
}

fn main() {
    let pair1 = Arc::new((Mutex::new(attendi::new()), Condvar::new()));
    let pair2 = pair1.clone();

    let h1 = thread::spawn(move ||{
        let (lk, cv) = &*pair1;
        let mut ll = lk.lock().unwrap();
        ll = cv.wait(ll).unwrap();
        println!("Thread (1).Aspetto...");
        thread::sleep(Duration::from_secs(2));
        println!("...ora posso partire!");
    });

    let h2 = thread::spawn(move ||{
        let (lk, cv) = &*pair2;
        let mut guard = lk.lock().unwrap();
        println!("Sono il secondo thread e parto prima del primo");
        println!("Aspetto un po");
        thread::sleep(Duration::from_secs(1));
        println!(".");
        thread::sleep(Duration::from_secs(1));
        println!(".");
        thread::sleep(Duration::from_secs(1));
        println!(".");
        guard.flag=true;
        cv.notify_one();
    });

    //Si aspetta la terminazione dei thread 'spawnati'
    _ = h1.join();
    _ = h2.join();
}
