use std::sync::mpsc::{channel, sync_channel};
use std::thread;
use std::time::Duration;
use tde_exchanger::exchange::Exchanger;


#[test]
fn test_exchange_single_value() {
    let (tx1, rx1) = channel::<i32>();
    let (tx2, rx2) = channel::<i32>();

    let p1 = Exchanger::new(tx1, rx2);
    let p2 = Exchanger::new(tx2, rx1);

    let h1 = thread::spawn(move||{
        assert_eq!(Some(3), p1.exchange(4));
    });

    let h2 = thread::spawn(move||{
        assert_eq!(Some(4), p2.exchange(3));
    });
    h1.join().unwrap();
    h2.join().unwrap();
}

#[test]
fn test_exchange_multiple(){
    let (tx1, rx1) = channel::<i32>();
    let (tx2, rx2)=channel::<i32>();
    let p1 = Exchanger::new(tx1, rx2);
    let p2=Exchanger::new(tx2, rx1);

    let t1 = thread::spawn(move||{
        assert_eq!(Some(3), p1.exchange(4));
        assert_eq!(Some(0), p1.exchange(1));
    });

    let t2 = thread::spawn(move||{
        assert_eq!(Some(4),p2.exchange(3));
        assert_eq!(Some(1), p2.exchange(0));
    });
}

#[test]
fn test_exchange_error(){
    let (tx1, rx1) = channel::<i32>();
    let (tx2, rx2)=channel::<i32>();
    let p1 = Exchanger::new(tx1, rx2);
    let p2=Exchanger::new(tx2, rx1);

    drop(p2);
    let h1 = thread::spawn(move||{
       assert_eq!(None, p1.exchange(2));
    });
}

