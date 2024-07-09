use tde_cache::cache::Cache;
use std::sync::Arc;

#[cfg(test)]

#[test]
fn test(){
    let cache = Cache::<i32, i32>::new();
    let square = |i: i32| {
      i*i
    };

    let a = cache.get(2, square);
    assert_eq!(*a, 4);
    //assert_eq!(*(cache.get(2, square)), 4);
}



