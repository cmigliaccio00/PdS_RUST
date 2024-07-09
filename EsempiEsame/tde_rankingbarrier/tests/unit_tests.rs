use std::collections::HashSet;
use std::sync::Arc;
use std::thread;
use tde_rankingbarrier::ranking::{RankingBarrier, Stato};

#[cfg(test)]

#[test]
fn test_ranking(){
    let rb = Arc::new(RankingBarrier::new(5).unwrap());
    let mut handlers = Vec::new();

    for i in 1..=5{
        let rb1 = rb.clone();
        handlers.push(thread::spawn(move||{
            //Ordine di entrata...
            rb1.wait()
        }))
    }
    let mut ris:HashSet<i32> = handlers.into_iter()
        .map(|f|f.join().unwrap())
        .collect();

    assert_eq!(ris.eq(&HashSet::from([1,2,3,4,5])), true);
}

#[test]
fn test_reset_count_state(){
    let rb = Arc::new(RankingBarrier::new(5).unwrap());
    let mut handlers = Vec::new();

    for i in 1..=5{
        let rb1 = rb.clone();
        handlers.push(thread::spawn(move||{
            //Ordine di entrata...
            rb1.wait()
        }))
    }

    handlers.into_iter().for_each(|el|{
        el.join().unwrap();
    });
    let (cnt, state) = rb.get_tupla();

    assert_eq!(state, Stato::Ingresso);
    assert_eq!(cnt, 0);
}

#[test]
fn test_ciclicity(){
    let rb = Arc::new(RankingBarrier::new(5).unwrap());
    for _ in 1..10{
        let mut handlers = Vec::new();
        for i in 1..=5{
            let rb1 = rb.clone();
            handlers.push(thread::spawn(move||{
                //Ordine di entrata...
                rb1.wait()
            }))
        }
        let mut ris:HashSet<i32> = handlers.into_iter()
            .map(|f|f.join().unwrap())
            .collect();

        assert_eq!(ris.eq(&HashSet::from([1,2,3,4,5])), true);
    }
}





