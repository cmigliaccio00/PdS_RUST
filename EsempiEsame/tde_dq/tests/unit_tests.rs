use std::ops::Add;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tde_dq::dq::DelayedQueue;

#[test]
fn test_size(){
    let dq = DelayedQueue::<i32>::new();
    assert_eq!(dq.size(),0);

}

#[test]
fn test_offer(){
    let dq = DelayedQueue::<i32>::new();
    dq.offer(Instant::now(), 14);
    assert_eq!(dq.size(), 1);
}

#[test]
fn test_take(){
    let dq = DelayedQueue::<i32>::new();

    let i = Instant::now()+Duration::from_secs(3);
    dq.offer(i, 5);
    assert_eq!(dq.take().unwrap(), 5);
}

#[test]
fn test_take_multiple(){
    let dq1 = Arc::new(DelayedQueue::<i32>::new());
    let dq2 = dq1.clone();

    dq1.offer(Instant::now().add(Duration::from_secs(3)), 2);
    dq1.offer(Instant::now().add(Duration::from_secs(1)), 5);

    let h = thread::spawn(move||{
        assert_eq!(dq2.take().unwrap(), 5);
        assert_eq!(dq2.take().unwrap(), 2);
    });
    _=h.join();     //Aspetto la fine del thread creato

    //Ho estratto tutti gli elementi --> take dovrebbe fallire!
    assert_eq!(dq1.take().is_none(), true);
}






