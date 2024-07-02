use std::sync::Arc;
use std::thread;
use tde_mpmc::mpmc::MpMc;

#[test]
fn try_create(){
    let my_mpmc = MpMc::<i32>::new(10);
    assert_eq!(my_mpmc.chan_size(), 0);
}


/* Provo a mandare un messaggio sul canale, se tutto va bene:
    - la dimensione del canale è cresciuta di uno
    - dovrei ottenere dalla send un () */
#[test]
fn try_send(){
    let my_mpmc = MpMc::<i32>::new(10);
    let res=my_mpmc.send(5);
    assert_eq!(res.unwrap(), ());
    assert_eq!(my_mpmc.chan_size(), 1);
}

/* Mando un singolo messaggio sul canale, provo a riceverlo
e testo il risultato di tale operazione */
#[test]
fn try_recv(){
    let my_mpmc = MpMc::<i32>::new(10);
    my_mpmc.send(4).unwrap();               //Metto sul canale
    assert_eq!(my_mpmc.chan_size(), 1);
    //Prelevo dal canale
    assert_eq!(my_mpmc.recv().unwrap(), 4);
    assert_eq!(my_mpmc.chan_size(),0);
}

/* Provo a mettere sul canale piu' di un elemento e
in un altro thread a riceverli. Il controllo viene fatto sul
conteggio di messaggi ricevuti */
#[test]
fn try_recv_multiple(){
    let my_mpmc = Arc::new(MpMc::<i32>::new(10));
    let mpmc1 = my_mpmc.clone();
    //Thread principale: mando due elementi sul canale
    my_mpmc.send(4);    my_mpmc.send(8);

    let handler = thread::spawn(move||{
        assert_eq!(mpmc1.recv().unwrap(), 4);
        assert_eq!(mpmc1.recv().unwrap(), 8);
        //Se tutto va bene, dovrei contare qui 2 elementi
    });

   _=handler.join();
}

/* Provo a mandare un messaggio prima e dopo che il canale
sia stato chiuso. Mi aspetto di ricevere prima Some, dopo None */
#[test]
fn try_close_and_send(){
    let mp = Arc::new(MpMc::<i32>::new(10));
    let mp1 = mp.clone();

    //Mando prima di chiudere --> Mi aspetto sia tutto ok
    assert_eq!(mp.send(4), Some(()));
    assert_eq!(mp.chan_size(), 1);
    //Chiudo il canale, poi mando --> Mi aspetto di ricevere None

    let h = thread::spawn(move||{
        let ris = mp1.shutdown();
        assert_eq!(ris, Some(()));
    });
    _=h.join();

    //Provo di nuovo a mandare
    assert_eq!(mp.send(3), None);
}
/*
#[test]
fn try_insert_multi_thread(){
    let my = Arc::new(MpMc::<i32>::new(8));
    let mut handlers = Vec::new();

    for i in 0..10{
        let my1 = my.clone();
        handlers.push(thread::spawn(move||{
            //Mando un messaggio sul canale
            _=my1.send(i);
        }));
    }
    //Aspetto i thread creati
    for i in 0..10{
        _=handlers.pop().unwrap().join();
    }
    //Controllo che la dimensione del canale sia giusta
    assert_eq!(my.chan_size(), 10);
}
*/

/* Provo a mettere sul canale un certo numero di messaggi, mi accerto
poi di riceverli tutti anche una volta che il canale sia stato chiuso */
#[test]
fn try_close_and_recv(){
    let my_mpmc1 = Arc::new(MpMc::<i32>::new(10));
    let my_mpmc2 = my_mpmc1.clone();

    //Mando 5 messaggi sul canale
    for i in 0..5{
        my_mpmc1.send(i);
    }
    assert_eq!(my_mpmc1.chan_size(),5);

    //Un thread legge due messaggi
    let h1 = thread::spawn(move||{
        _=my_mpmc2.recv();
        _=my_mpmc2.recv();
        assert_eq!(my_mpmc2.chan_size(),3);
    });
    //Attendo che questo finisca...
    _=h1.join();
    assert_eq!(my_mpmc1.shutdown(), Some(()));      //...chiudo il canale

    //Provo a leggere ancora dal canale
    let mut cont = 0;
    while let Some(ris) = my_mpmc1.recv(){
        cont = cont + 1;
    }
    //Dovrebbero essere 5-2 letti prima =3
    assert_eq!(cont, 3);
}


#[test]
fn try_mutex_error(){
    let my = Arc::new(MpMc::<i32>::new(4));
    let my1 = my.clone();
    let h = thread::spawn(move||{
        let dato=my.chan.lock();
        panic!("AAAAAACCCC");
    });
    let a=h.join();
    assert_eq!(a.is_err(),true);

    //A regola dovrebbe fallire la chiusura del canale
    //avendo avvelenato il lock
    assert_eq!(my1.shutdown(), None);
}