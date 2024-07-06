use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn genera_messaggi(s: Sender<String>){
    thread::spawn(move||{
        for i in 1..10{
            thread::sleep(Duration::from_secs(1));
            s.send(format!("Ciao sono {i}")).unwrap();
        }
    });
}

fn start_process(s: Sender<String>, r: Receiver<String>){
    //Creazione del processo
    let figlio = Command::new("cmd")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Nessun processo creato!");

    //Creo il thread che fa da dispatcher
    thread::spawn(move||{
        let mut in_flow = BufReader::new(figlio.stdout.unwrap());
        let mut out_flow = figlio.stdin.unwrap();

        for riga in r.iter(){
            //Mando al processo
            out_flow.write_all(riga.as_bytes()).unwrap();
            println!("{riga}");
            let mut buffer = String::new();
            //Aspetto che il processo mi mandi qualcosa
            let line = in_flow.read_line(&mut buffer);

            match line{
                Ok(_)=> {
                    //Riga letta correttamente, la mando verso il receiver
                    s.send(buffer).unwrap();
                    continue;
                },
                Err(_) => {
                    println!("errore di lettura dalla pipe del processo!");
                }
            }
        }
    });
}

fn main(){
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    //genero i messaggi (thread)
    genera_messaggi(tx1);
    //Li mando al processo-->Il processo li elabora-->li rimando indietro (thread)
    start_process(tx2,rx1);
    //Li ricevo e ne faccio qualcosa (thread principale)
    rx2.iter().for_each(|riga| {
        println!("Riga ricevuta! Contenuto: {riga}");
    });
}
