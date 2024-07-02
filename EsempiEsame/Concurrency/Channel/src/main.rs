use std::sync::mpsc::channel;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Debug)]
struct Messaggio{
    id: usize,
    text: String,
    length: usize,
}

impl Messaggio{
    fn new(id: usize, t:String, l:usize)->Self{
        Self{id, text: t, length:l}
    }

    fn read_msg(&self){
        println!("-----------NUOVO MESSAGGIO--------------");
        println!("ID messaggio: {}", self.id);
        println!("Lunghezza messaggio: {}", self.length);
        println!("Testo: {}", self.text);
    }
}

struct Conteggio{
    quanto: usize
}

impl Conteggio{
    fn new()->Self{
        Self{quanto:0}
    }

    fn inc(&mut self){
        self.quanto +=1;
    }
}

fn main() {
    //Creo il canale di comunicazione
    let (Tx, Rx) = channel();
    let Tx1 = Tx.clone();
    let Tx2 = Tx.clone();

    let conta1 = Arc::new(Mutex::new(Conteggio::new()));
    let conta2 = conta1.clone();

    //Ricorda Sender si può clonare, Receiver NO!
    //Siamo in un paradigma mpsc --> multiple producer single consumer

    let hprod1 = thread::spawn(move ||{
        thread::sleep(Duration::from_secs(3));
        let mut dato = conta1.lock().unwrap();
        dato.inc();
        let myStr = "Messaggio da Produttore #1: Mandato!".to_string();
        let len = myStr.len();
        println!("Messaggio #{}", dato.quanto);
        _=Tx1.send(Messaggio::new(dato.quanto,
                                  myStr,
                                  len));

    });

    let hprod2 = thread::spawn(move ||{
        let mut dato = conta2.lock().unwrap();
        dato.inc();
        let myStr= "Messaggio da Produttore #2: Mandato! ".to_string();
        let len = myStr.len();
        println!("Messaggio #{}", dato.quanto);
        _=Tx2.send(Messaggio::new(dato.quanto,
                                  myStr,
                                  len));
    });

    drop(Tx);

    let hcons = thread::spawn(move ||{
        while let Ok(M) = Rx.recv(){
            println!("Receiver dice: RICEVUTO!");
            M.read_msg();
            println!(" ");
        }
    });

    _=hprod1.join();
    _=hprod2.join();
    _=hcons.join();
}
