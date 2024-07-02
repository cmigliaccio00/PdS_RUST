use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread;
use tde_multichannel::multichannel::MultiChannel;
#[test]
fn test_creazione(){
    let mc = MultiChannel::new();
    assert_eq!(mc.num_sub(), 0);
}

#[test]
fn test_iscrizione(){
    let mc = MultiChannel::new();
    let r = mc.subscribe();
    assert_eq!(mc.num_sub(),1);
}

#[test]
fn test_send(){
    //Test OK
    let mc = MultiChannel::new();
    _=mc.subscribe();       //iscrivo un utente
    assert_eq!(mc.send(3), Ok(()));
    drop(mc);

    //Test Error (nessun iscritto)
    let mc = MultiChannel::new();
    assert_eq!(mc.send(0).is_err(), true);
}

#[test]
fn test_recv_single_thread(){
    let mc = Arc::new(MultiChannel::new());
    let mc1 = mc.clone();
    let r=mc.subscribe();
    let h1 = thread::spawn(move||{
       _=mc1.send(4);
    });
    assert_eq!(r.recv().unwrap(), 4);
}


#[test]
fn test_recv_multi_thread(){
    let mc1 = Arc::new(MultiChannel::new());
    let mc2 = mc1.clone();

    let r1 = mc1.subscribe();
    let r2 = mc1.subscribe();

    let h1 = thread::spawn(move ||{
        mc2.send(4);
    });

    assert_eq!(r1.recv().unwrap(), 4);

    //Mando un datos
    drop(mc1);

    _=h1.join();
}



