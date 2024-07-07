use std::sync::Arc;
use std::thread;
use tde_execlimiter::exec_lim::ExecutionLimiter;

#[test]
fn test_init(){
    let e  = ExecutionLimiter::new(5);
    assert_eq!(e.get_conteggio(), 0);
    let ris = e.execute(||5).unwrap();
    assert_eq!(ris, 5);
}

#[test]
fn test_exec(){
    let e = ExecutionLimiter::new(5);
    let ris = e.execute(||false).unwrap();
    assert_eq!(ris, false);
}

#[test]
fn test_failed(){
    let e= ExecutionLimiter::new(3);
    assert_eq!(e.execute(||5).unwrap(), 5);

    //Forzo una esecuzione che sicuramente fallirà
    let ris = e.execute(|| {panic!();});
    assert_eq!(ris.is_none(), true);
}

#[test]
fn test_pool(){
    let e = Arc::new(ExecutionLimiter::new(4));
    let mut handlers = Vec::new();
    let f = || {5+1};

    for _ in 0..4{
        let exec = e.clone();
        handlers.push(thread::spawn(move||{
            exec.execute(f);
        }));
    }

    for h in handlers{
        h.join().unwrap();
    }
}