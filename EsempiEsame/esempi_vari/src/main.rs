//Quesiti TdE Programmazione di Sistema - 25 Giugno 2024

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
struct S{
    i: i32,
}

impl From<i32> for S { fn from(value:i32) ->Self{Self{i:value}}}

fn quesito1(){
    let numbers = vec![1,2,3,4,5,8];

    let res = numbers.
        iter().
        filter(|&x|{x%2==0}).
        zip('a'..'z');

    let last = res
        .clone()
        //Trasformo una tupla in stringa tramite la macro format!(...)
        .map(|(a,b)|{format!("{b}{a}")})
        .last();
    println!("last: {:?}", last);               //stampa c8
    println!("res: {:?}", res.count());         //stampa 3
}

fn quesito2(){
    let pair1 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair1.clone();
    thread::spawn(move||{
        //Questo thread notifica
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started=true;
        cvar.notify_all();
    });
    //Il thread principale aspetta
    println!("Sto aspettando....");
    thread::sleep(Duration::from_secs(1));          //Aspetto per un secondo!

    let (lock, cvar) = &(*pair1);
    let mut started = lock.lock().unwrap();
    started = cvar.
        //Se non ci mettessi la wait_while, dato che attendo 1s prima di procedere
        //mi perdo la notifica che mi arriva dal thread spawnnato in precedenza.
        wait_while(started, |s|*s==false)
        .unwrap();

    println!("End");
}

fn quesito3(){
    let mut v= Vec::<S>::new();
    let s = 42.into();

    for i in 0..3{
        v.push(s);
    }
    println!("{:?}", v);
}

fn main(){
    //quesito1();
    //quesito2();
    quesito3()
}