use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tde_dispatcher::disp::{Subscription, Dispatcher};
#[cfg(test)]

#[test]
fn test_subscribe(){
    let disp = Dispatcher::<String>::new();
    let s1 = disp.subscribe();
    let s2 = disp.subscribe();
    assert_eq!(disp.get_dimqueue(),2);
}


#[test]
fn test_dispatch(){
    let disp = Arc::new(Dispatcher::<String>::new());
    let disp2 = Arc::clone(&disp);
    let disp1 = Arc::clone(&disp);

    let h1 = thread::spawn(move||{
        let s1 = disp1.subscribe();
        assert_eq!(s1.read().unwrap(), "Ciao, sono Carlo".to_string());
        assert_eq!(s1.read().unwrap(), "Altro messaggio".to_string());
    });

    let h2 = thread::spawn(move ||{
        let s2 = disp2.subscribe();
        assert_eq!(s2.read().unwrap(), "Ciao, sono Carlo".to_string());
    });

    let h3 = thread::spawn(move||{
        disp.dispatch("Ciao, sono Carlo".to_string());
        disp.dispatch("Altro messaggio".to_string());
        drop(disp);
    });

    //Attesa dei thread
    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
}

/*
#[test]
fn test_read_none(){
    let disp = Arc::new(Dispatcher::<String>::new());
    let disp1 = Arc::clone(&disp);

    let h1 = thread::spawn(move||{
        let s1 = disp1.subscribe();
        assert_eq!(s1.read(), None);
    });

    h1.join().unwrap();

    thread::sleep(Duration::from_secs(3));
    drop(disp);


}
*/

#[test]
fn test_survive_dispatcher(){
    let disp = Arc::new(Dispatcher::<String>::new());
    let disp1 = Arc::clone(&disp);
    let disp2 = Arc::clone(&disp);

    //Questo sottoscrittore continua a ricevere messaggi nonostante
    //un altro si sia 'suicidato'c
    let h1 = thread::spawn(move||{
        let s1 = disp1.subscribe();
        assert_eq!(String::from("Primo Messaggio"), s1.read().unwrap());
        assert_eq!(String::from("Secondo Messaggio"), s1.read().unwrap());
        assert_eq!(String::from("Terzo Messaggio"), s1.read().unwrap());
    });

    let h2 = thread::spawn(move ||{
        let s1 = disp2.subscribe();
        _=s1.read();
        drop(s1);
    });

    //Manda messaggi
    let h3 = thread::spawn(move||{
        disp.dispatch("Primo Messaggio".to_string());
        disp.dispatch("Secondo Messaggio".to_string());
        disp.dispatch("Terzo Messaggio".to_string());
    });

    h1.join().unwrap();
    h2.join().unwrap();
}

#[test]
fn test_survive_Subscriptor(){
    let disp = Arc::new(Dispatcher::<String>::new());
    let disp1 = Arc::clone(&disp);

    let h2 = thread::spawn(move||{
       disp.dispatch(String::from("Primo messaggio"));
       disp.dispatch(String::from("Secondo messaggio"));
       disp.dispatch(String::from("Terzo messaggio"));
        drop(disp);
    });

    let h1 = thread::spawn(move||{
        let s1 = disp1.subscribe();
        assert_eq!(String::from("Primo Messaggio"), s1.read().unwrap());
        thread::sleep(Duration::from_secs(5));
        assert_eq!(String::from("Secondo Messaggio"), s1.read().unwrap());
        assert_eq!(String::from("Terzo Messaggio"), s1.read().unwrap());
        drop(disp1);


    });

    h1.join().unwrap();
    h2.join().unwrap();
}









