pub mod multichannel{
    use std::sync::mpsc::{channel, Receiver, Sender, SendError};
    use std::sync::{Condvar, Mutex};

    pub struct MultiChannel{
        canali: Mutex<Vec<Sender<u8>>>
    }
    impl MultiChannel{
        pub fn new()->Self{
            Self{canali: Mutex::new(Vec::new())}
        }
        pub fn num_sub(&self)->usize{
            self.canali.lock().unwrap().len()
        }
        /*Mi iscrivo al canale, mi viene data */
        pub fn subscribe(&self)->Receiver<u8>{
            let (rx, tx) = channel();
            let mut canali =self.canali.lock().unwrap();
            canali.push(rx);
            tx
        }
        pub fn send(&self, data:u8)->Result<(),SendError<u8>>{
            let canali = self.canali.lock().unwrap();
            if (!canali.is_empty()){
                canali.iter().for_each(|e|{
                    _=e.send(data);
                });
                //Se ci sono sottoscrittori
                return Ok(())
            }
            return Err(SendError(data))
        }
    }
}