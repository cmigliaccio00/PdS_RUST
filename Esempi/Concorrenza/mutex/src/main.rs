use std::sync::{Arc, Mutex};
use std::thread;

struct Contatore{
    numero: u32
}

impl Contatore{
    fn new()->Self{
        Contatore{numero:0}
    }

    fn add(&mut self){
        self.numero+=10;
    }
}

//Si vuole implementare l'incremento di una variabile in
//modo atomico, fatto con l'utilizzo dei Mutex
fn main() {
    let m = Arc::new(Mutex::new(Contatore::new()));

    //Creo le copie da passare ai thread
    let m1 = Arc::clone(&m);
    let m2 = Arc::clone(&m);

    let t1 = thread::spawn(move||{
        if let Ok(mut conta) = m1.lock(){
            //Incremento il contatore
            conta.add();
        }
    });

    let t2  = thread::spawn(move||{
        if let Ok(mut conta) = m2.lock(){
            //Incremento il contatore
            conta.add();
        }
    });

    //Aspetto la terminazione dei due thread
    t1.join().unwrap();
    t2.join().unwrap();

    //Stampo il valore finale del contatore
    println!("Valore finale contatore: {}", m.lock().unwrap().numero);
}
