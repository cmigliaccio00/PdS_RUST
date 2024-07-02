use std::sync::Arc;
use std::thread;
use std::sync::Mutex;

struct Intero{
    valore: i32
}

impl Intero{
    fn new(val: i32)->Self{
        Self{valore: val}
    }

    fn add(&mut self){
        self.valore = self.valore + 1;
    }
}

//Uso di un mutex in accoppiata con una struttura
fn main() {
    //Struttura da accedere in mutua esclusione
    let myInt = Intero::new(0);
    let lock1 = Arc::new(Mutex::new(myInt));
    let lock2 = lock1.clone();
    let lock3 = lock1.clone();

    let h1 = thread::spawn(move ||{
        let mut guard = lock1.lock().unwrap();
        for _ in 1..=10{
            guard.add();
        }
    });

    let h2 = thread::spawn(move ||{
        let mut guard = lock2.lock().unwrap();
        for _ in 1..=10{
            guard.add();
        }
    });

    //Aspetto la terminazione dei due thread creati
    _= h1.join();
    _= h2.join();

    let guard = lock3.lock().unwrap();
    println!("Valore finale: {}",guard.valore);
}
