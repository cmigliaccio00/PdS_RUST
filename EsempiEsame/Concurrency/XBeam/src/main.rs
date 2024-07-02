use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use crossbeam::channel;
use crossbeam_channel::{unbounded};

//----------------------------------------------------
struct Messaggio{
    testo: String
}

impl Messaggio{
    fn new(testo:String)->Self{
        Self{testo}
    }

    fn get_testo(&self)->&str{
        &self.testo
    }
}
//------------------------------------------------------

fn main() {
    //Channel Prod-->Stage1
    let (TX1, RX1) = unbounded::<Messaggio>();
    //Channel Stage1-->Stage2
    let (TX2, RX2) = unbounded::<Messaggio>();
    //Channel Stage2-->Cons
    let (TX3, RX3)=unbounded::<Messaggio>();

    //Produttore --> Produci messaggio
    let myTX = TX1.clone();
    let prod_h = thread::spawn(move || {
        println!("Invio di:    PrOva di testo con me            ");
        _=myTX.send(Messaggio::new("   PrOva di testo con me            ".to_string()));
        thread::sleep(Duration::from_secs(2));
        println!("Invio di:    ... la strinGa continua qui -->   ");
        _=myTX.send(Messaggio::new("   ... la strinGa continua qui -->   ".to_string()));
        thread::sleep(Duration::from_secs(1));
        println!("Invio di:   ...E anche qUi!!  ");
        _=myTX.send(Messaggio::new("  ...E anche qUi!!".to_string()));
        thread::sleep(Duration::from_secs(1));
    });

    //Stage 1 --> TOGLI SPAZI
    //ricevo da RX1
    let my_rx = RX1.clone();
    //trasmetto su TX2
    let my_tx = TX2.clone();
    let stag1_h  = thread::spawn(move ||{
        //---------------------------------------------------------
        while let Ok(a) = my_rx.recv(){
            println!{"Primo STAGE"};
            let a= a.get_testo().trim();       //tolgo spazi
            thread::sleep(Duration::from_secs(2));
            _=my_tx.send(Messaggio::new(a.to_string()));                  //trasmetto
        }

    });

    //Stage2 --> tutto maiuscolo
    let my_rx = RX2.clone();
    let my_tx = TX3.clone();
    let stag2_h = thread::spawn(move||{
        //ricevo la stringa --> la trasformo (TUTTO MAIUSCOLE)
        while let Ok(stringa) = my_rx.recv(){
            println!("Secondo STAGE");
            let stringa=stringa.get_testo().
                to_uppercase();
            thread::sleep(Duration::from_secs(2));
            _=my_tx.send(Messaggio::new(stringa.to_string()));
        }
    });

    //Consumatore --> Stampa messaggio
    let my_rx = RX3.clone();
    let cons_h  = thread::spawn(move ||{
        while let Ok(ris) = my_rx.recv(){
            let ris = ris.get_testo();
            println!("Ricevo risultato: {}", ris);
            println!(" ");
        }
    });

    drop(TX1);
    drop(TX2);
    drop(TX3);

    _=prod_h.join();
    _=stag1_h.join();       _=stag2_h.join();
    _=cons_h.join();
}
