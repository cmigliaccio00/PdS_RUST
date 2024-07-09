pub mod disp{
    use std::iter::Rev;
    use std::sync::mpsc::{channel, Receiver, Sender, SendError};
    use std::sync::Mutex;

    //----------------Tipo 'Subscription'----------------
    pub struct Subscription<Msg: Clone>{
        rx: Receiver<Msg>
    }
    impl<Msg: Clone> Subscription<Msg>{
        pub fn new(rx: Receiver<Msg>)->Self{
            Self{rx}
        }
        pub fn read(&self)->Option<Msg>{
            let a = self.rx.recv().ok()?;
            Some(a)
        }
    }

    pub struct Dispatcher<Msg: Clone>{
        //lista dei sottoscrittori
        tx_queue: Mutex<Vec<Sender<Msg>>>
    }

    impl<Msg: Clone> Dispatcher<Msg> {
        pub fn new() -> Self {
            Self { tx_queue: Mutex::new(Vec::new()) }
        }

        //Iscrizione
        pub fn subscribe(&self) -> Subscription<Msg> {
            let (tx, rx) = channel();
            let mut coda = self.tx_queue.lock().unwrap();
            coda.push(tx);
            Subscription::new(rx)
        }

        //Invio di un messaggio agli iscritti
        pub fn dispatch(&self, msg: Msg) {
            //Acquisisco lock sulla coda
            let coda = self.tx_queue.lock().unwrap();
            //Distribuisco i messaggi
            for el in coda.iter() {
                 el.send(msg.clone());
            }
        }

        pub fn get_dimqueue(&self) -> usize {
            self.tx_queue.lock().unwrap().len()
        }
    }
}