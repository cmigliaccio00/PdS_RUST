use std::cmp::Ordering;
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};
pub struct DelayedQueue<T>{
    queue: Mutex<Vec<(Instant, T)>>,
    cvar: Condvar
}

impl<T> DelayedQueue<T>{
    pub fn new() -> Self{
        Self{queue: Mutex::new(Vec::new()),
            cvar: Condvar::new()}
    }

    pub fn size(&self)->usize{
        self.queue.lock().unwrap().len()
    }

    pub fn offer(&self, t: T, i: Instant){
        let mut coda = self.queue.lock().unwrap();
        coda.push(t);
        coda.sort_by(|&el, &el2|{
           if el.0 > el2.1{
               Ordering::Greater
           }
            else if el.0 < el2.1{
                Ordering::Less
            }
            else { Ordering::Equal }
        });
        self.cvar.notify_all();
    }

    pub fn take(&self)->Option<T>{
        let mut queue = self.queue.lock().unwrap()

        loop{
            let to_wait;

            match queue.last(){
                None => return None,
                Some(min) => {
                    if min.0.duration_since(Duration::now()) == Duration::from_nanos(0){

                    }
                    else {to_wait = min.0.duration_since(Instant::now()) - Duration::from_nanos(0)}
                }
            }
        }
    }
}

fn main() {

}
