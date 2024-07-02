pub mod mpmc{
    use std::collections::VecDeque;
    use std::sync::{Condvar, LockResult, Mutex};

    //Tipo 'Canale'
    pub struct Canale<E: Send>{
        data: VecDeque<E>,
        closed: bool,
    }
    impl<E: Send> Canale<E>{
        fn new(n: usize)->Self{
            Self{data: VecDeque::with_capacity(n), closed:false}
        }
        fn close(&mut self)                 { self.closed = true;                       }
        fn closed(&self)->bool              { self.closed                               }
        fn add_item(&mut self, item: E)     { self.data.push_back(item);                }
        fn rem_item(&mut self)->E           { self.data.pop_front().unwrap()            }
        fn is_full(&self)->bool             { self.data.len() == self.data.capacity()   }
        fn is_empty(&self)->bool            { self.data.len()==0                        }
        fn length(&self)->usize             { self.data.len()                           }
    }

    //Tipo MpMc
    pub struct MpMc<E> where E:Send{
        pub chan: Mutex<Canale<E>>,
        cv: Condvar,
    }

    impl<E:Send> MpMc<E>{
        pub fn new(n: usize)->Self{
            Self{chan: Mutex::new(Canale::new(n)), cv:Condvar::new()}
        }

        pub fn chan_size(&self)->usize{
            self.chan.lock().unwrap().length()
        }

        //Inserimento di un elemento nel canale
        pub fn send(&self, e: E)->Option<()>{
            //Prendo possesso del canale
            let mut canale = self.chan.lock().unwrap();

            canale = self.cv.wait_while(canale, |canale|{
                //Aspetto finché il canale è aperto ed è pieno
                !canale.closed && canale.is_full()
            }).unwrap();

            if !canale.closed{
                canale.add_item(e);
                return Some(())
            }
            None
        }

        //Estrazione di un elemento dal canale
        pub fn recv(&self)->Option<E>{
            //Prendo possesso del canale
            let mut canale = self.chan.lock().unwrap();

            //Mi metto in attesa se serve
            canale = self.cv.wait_while(canale, |c|{
                c.is_empty() && !c.closed()
            }).unwrap();

            if canale.is_empty(){
               return None
            }

            //Se il canale non è vuoto, seppure dovesse
            //essere chiuso estraggo elementi finché posso
            let el = canale.rem_item();
            self.cv.notify_all();
            Some(el)
        }

        //Chiusura del canale + gestione Errore
        pub fn shutdown(&self)->Option<()>{
            //Prendo possesso del canale
            let canale = self.chan.lock();

            //Gestisco la situazione in cui ci fosse avvelenamento del Mutex
            match canale{
                LockResult::Err(er) =>{
                    None
                },
                LockResult::Ok(mut guard) =>{
                    guard.close();
                    self.cv.notify_all();
                    Some(())
                }
            }
        }
        //-----------------------------------------------------------------------

    }
}