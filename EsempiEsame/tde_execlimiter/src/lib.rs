pub mod exec_lim{
    use std::sync::{Condvar, Mutex};
    use std::thread;
    use std::time::Duration;

    pub struct ExecutionLimiter{
        conta: Mutex<i32>,
        cvar: Condvar,
        limite: i32,
    }

    impl ExecutionLimiter{
        pub fn new(limite: i32)->Self{
            Self{
                conta: Mutex::new(0),
                cvar: Condvar::new(),
                limite
            }
        }

        ///Funzione non richiesta! (Per fare i test)
        pub fn get_conteggio(&self)->i32{
            *self.conta.lock().unwrap()
        }

        pub fn execute<R: Send+'static, Q:Send+'static>(&self,f: Q)->Option<R> where Q:Fn()->R{
            //Acquisisco il lock
            let mut dato= self.conta.lock().unwrap();

            println!("Sbloccato {}", dato);

            //Ho raggiunto il limite di esecuzioni? Aspetta senza consumare CPU
            dato = self.cvar.wait_while(dato, |d|{
                *d==self.limite
            }).unwrap();

            //Aggiorno il conteggio
            *dato = *dato + 1;
            let h = thread::spawn(f);
            let res = h.join();
            let ris = match res {
                Ok(ris) => {

                    Some(ris)
                },
                Err(_) => {
                    *dato = *dato - 1;
                    self.cvar.notify_all();
                     None
                }
            };

            ris
        }
    }
}