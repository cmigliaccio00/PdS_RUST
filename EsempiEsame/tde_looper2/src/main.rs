use std::sync::mpsc::{channel, Sender};
use std::thread;

struct Looper<Message:Send>{
    sender: Sender<Message>
}

impl<Message:Send+'static> Looper<Message>{
    fn new<F: Fn(Message)->()+Send+'static,
        V:Fn()->()+Send+'static>(process: F, cleanup: V) ->Self{
        let (rx, tx) = channel();
        //Quci creo il thread
        thread::spawn(move||{
            loop{
                match tx.recv(){
                    Ok(msg)=>{
                       process(msg);
                    },
                    Err(_)=> break
                }
            }
            cleanup();
        });
        Self{sender:rx}
    }

    pub fn send(&self, msg: Message){
        self.sender.send(msg).unwrap()
    }
}

//Siamo sicuri che non aspettiamo in eterno:
//quando l'istanza della struttura esce dallo scope
//la struttura stessa viene distrutta
impl<Message: Send> Drop for Looper<Message>{
    fn drop(&mut self) {
        let a =self.sender.to_owned();
        drop(a);
    }
}

fn main() {
    let looper = Looper::<i32>::new(
        |msg|{println!("{msg}")},
        ||{println!("Ciao")});

    looper.send(12);
}
