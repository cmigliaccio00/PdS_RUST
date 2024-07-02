pub mod dq{
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::sync::{Condvar, Mutex};
    use std::time::Instant;

    #[allow(dead_code)]
    struct Item<T:Send>{
        i: Instant,
        t: T,
    }
    //Bisogna implementare i tratti Ord->PartialOrd->Eq->PartialEq
    impl<T:Send> Item<T>{
        fn get_i(&self)->Instant    {   self.i  }
    }
    
    impl<T:Send> PartialOrd for Item<T>{
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(other.i.cmp(&self.i))
        }
    }

    impl<T: Send> Ord for Item<T>{
        fn cmp(&self, other: &Self) -> Ordering {
            other.i.cmp(&self.i)
        }
    }

    impl<T: Send> PartialEq for Item<T>{
        fn eq(&self, other: &Self) -> bool {
            self.i.eq(&other.i)
        }
    }

    impl<T: Send> Eq for Item<T>    {   }
    
    pub struct DelayedQueue<T:Send>{
        data: Mutex<BinaryHeap<Item<T>>>,
        cv: Condvar,
    }
    impl<T:Send> DelayedQueue<T>{
        pub fn new()->Self{
            Self{data: Mutex::new(BinaryHeap::new()),cv:Condvar::new()}
        }
        
        ///Inserisce nella DelayedQueue<T> un elemento caratterizzato dalla coppia (i,t)
        pub fn offer(&self, i: Instant, t: T){
            let mut pq = self.data.lock().unwrap();
            //Nota che: questo richiede che Item implementi il tratto Ord
            pq.push(Item{i,t});
            //Bisogna notificare che è stato inserito un nuovo elemento
            self.cv.notify_all();
        }
        
        /// Estrae l'elemento a scadenza più ravvicinata dalla coda. Lo ritorna se il periodo
        /// è passato, altrimenti aspetto che trascorra l'intervallo di tempo che ancora manca
        /// per poi eventualmente ritornarlo.
        pub fn take(&self)->Option<T>{
            let mut pq = self.data.lock().unwrap();

            while !pq.is_empty(){
                //Sono sicuro di poter fare unwrap()....
                let el = pq.peek().unwrap();
                let inst = el.get_i();

                if inst > Instant::now(){
                    //Estraggo l'elemento dalla collezione
                    // ritornando il campo T
                    return Some(pq.pop().unwrap().t)
                }
                //Mi metto in attesa che qualcosa cambi
                pq = self.cv.
                    wait_timeout(pq, inst.duration_since(Instant::now())).
                    unwrap().0;
                //Lo restituisce appena viene rifatta l'iterazione
            }
            None
        }

        ///Ritorna la dimensione della coda
        pub fn size(&self)->usize{
            self.data.lock().unwrap().len()
        }
    }
}