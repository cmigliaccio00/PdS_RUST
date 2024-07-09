pub mod exchange{
    use std::sync::mpsc::{Sender, Receiver};

    pub struct Exchanger<T: Send>{
        tx: Sender<T>,
        rx: Receiver<T>
    }

    impl<T:Send> Exchanger<T>{
        pub fn new(tx:Sender<T>, rx:Receiver<T>)->Self{
            Self{tx, rx}
        }

        pub fn exchange(&self, t: T)->Option<T>{
            let res_tx = self.tx.send(t).ok()?;
            let res_rx = self.rx.recv();
            match res_rx{
                Ok(ris) => Some(ris),
                Err(_)=>None
            }
        }
    }
}