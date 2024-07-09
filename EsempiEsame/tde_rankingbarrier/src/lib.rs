pub mod ranking{
    use std::ops::Deref;
    use std::sync::{Condvar, Mutex};

    #[derive(Eq,PartialEq, Copy, Clone, Debug)]
    pub enum Stato{
        Ingresso,
        Uscita
    }
    pub struct RankingBarrier{
        conta: Mutex<(i32, Stato)>,     //Quanti thread sono entrati al momento?
        limit: i32,
        cvar: Condvar,                  //Entrata
    }

    impl RankingBarrier{
        //Option mi permette di gestire il fatto
        //che se il parametro passato non è adeguato, non ci può essere
        pub fn new(limit: i32)->Option<Self>{
            if limit<2{
                return None
            }
            Some(Self{
                conta: Mutex::new((0, Stato::Ingresso)),
                limit,
                cvar: Condvar::new(),
            })
        }

       pub fn wait(&self)->i32{
           //Acquisisco il lock sulla coppia (Conteggio, Stato) --> Manca
           //il controllo if(Conteggio<limite)...
           let mut lock = self.conta.lock().unwrap();

           //---------------------Fase di Ingresso----------------------------
           (*lock).0 = (*lock).0 + 1;       //Il thread i-esimo entra
           let ris = (*lock).0;
           lock = self.cvar.wait_while(lock,|i|{
               (*i).0 < self.limit &&
                   (*i).1 == Stato::Ingresso
           }).unwrap();

           //-----------------Fase di Uscita--------------------
           //L'ultimo thread che arriva manda questa notifica per primo

           //Il PRIMO THREAD a far fallire la condizione sarà l'ultimo che entra
           //che ragionevolmente trova lo stato ancora in 'Stato::Ingresso'
           //quindi posso marcare lo stato come 'Stato::Uscita'
           if (*lock).1 ==Stato::Ingresso {
               //Cambio lo stato! Questo sblocca gli altri thread dall'attesa facendo fallire
               //il predicato nella wait_while
               (*lock).1 = Stato::Uscita;
           }

           //Sveglio tutti dall'attesa
           self.cvar.notify_all();

           //Decremento il contatore (questa operazione la fanno tutti)
           (*lock).0 = (*lock).0 - 1;
           println!("{}", (*lock).0);

           //Una volta ripristinato il conteggio a 0, devo assicurarmi che tutto sia
           //a posto per la prossima barriera, se lasciassi le cose come stanno
           //non riuscirei a far ripartire il tutto perché lo stato è 'Stato::Uscita'
           //Questa cosa però non posso farla subito perché altrimenti metterei di nuovo
           //in attesa i thread che sono usciti
           lock = self.cvar.wait_while(lock, |tupla|{
               (*tupla).0 > 0
           }).unwrap();

           //Questa operazione la farà solo il primo thread che esce dall'attesa
           if (*lock).1 == Stato::Uscita{
               self.cvar.notify_all();
               (*lock).1 = Stato::Ingresso;
           }

           println!("Sbloccato thread {}", ris);
           ris
       }

        pub fn get_tupla(&self)->(i32, Stato){
            let lock = self.conta.lock().unwrap();
            ((*lock).0, (*lock).1)
        }
    }


}