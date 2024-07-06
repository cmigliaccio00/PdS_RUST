pub mod looper{
    use std::collections::LinkedList;
    use std::fmt::{Debug, Display};
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;
    use std::time::Duration;

    #[derive(Clone)]
    pub struct Message<T: Send+Clone>{
        content: T
    }

    impl<T: Send+Clone> Message<T>{
        pub fn new(content:T)->Self{
            Self{content}
        }

        pub fn get_content(&self)->&T{
            &self.content
        }
    }

    struct Coda<T:Send+Clone>{
        queue: LinkedList<T>,
        ricevuti: Vec<T>
    }

    impl<T:Send+Clone> Coda<T>{
        fn new()->Self{
            Self{queue: LinkedList::new(), ricevuti: Vec::new()}
        }
    }

    pub struct Looper<T:Send+Clone>{
        pair: Arc<(Mutex<Coda<T>>, Condvar)>,
        process: fn(T)->(),
        cleanup: fn()->()
    }

    impl<T:Send +'static+Clone> Looper<T>{
        pub fn new(process: fn(T)->(), cleanup: fn()->())->Self{
            let a = Self{
                pair: Arc::new((Mutex::new(Coda::new()), Condvar::new())),
                process, cleanup
            };

            let pair2 = a.pair.clone();

            //Creo il thread che elaborerà i messaggi
            thread::spawn(move||{
                let (lock, cv) = &*pair2;
                let mut data = lock.lock().unwrap();
                //Mi metto in attesa di un messaggio
                data = cv.wait_while(data,
                                     |q|
                                     {q.queue.len()==0}).unwrap();

                //Estraggo l'elemento
                let el = data.queue.pop_front().unwrap();
                data.ricevuti.push(el.clone());
                process(el);
             });

            a
        }

        //Unico metodo pubblico che mi permette di inviare un messaggio
        pub fn send(&self, msg: T){
            let (lk, cv) = &*self.pair;
            let mut coda = lk.lock().unwrap();
            coda.queue.push_back(msg);
            cv.notify_all();
        }

        //Solo per fare un test
        pub fn get_ricevuti(&self)->Vec<T>{
            let  (lk, cv) = &*self.pair;
            let mut data = lk.lock().unwrap();
            data.ricevuti.clone()
        }
    }

    //Qui bisogna utilizzare il metodo
    impl<T:Send+Clone> Drop for Looper<T>{
        fn drop(&mut self) {
            //Invoco la funzione che mi è stata chiesta
            let a = self.cleanup;
            thread::sleep(Duration::from_secs(3));
            a();
        }
    }
}