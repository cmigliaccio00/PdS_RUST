pub mod cache{
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::sync::{Arc, Mutex};

    pub struct Cache<K: Clone+Hash+Eq,V:Clone>{
        cache: Mutex<HashMap<K,Arc<V>>>
    }

    impl<K: Clone+Hash+Eq, V:Clone> Cache<K,V>{
        pub fn new()->Self{
            Self{cache: Mutex::new(HashMap::new())}
        }

        pub fn get<Q: FnOnce(K)->V>(&self, k: K, f: Q)->Arc<V>{
            //Accedo in mutua esclusione alla mappa
            let mut cache = self.cache.lock().unwrap();

            let ris = cache.get(&k);
            if let Some(val) = ris{
                return val.clone();
            }
            cache.insert( k.clone(), Arc::new(f(k.clone())));
            let myarc = cache.get(&k).unwrap();
            myarc.clone()
        }
    }
}