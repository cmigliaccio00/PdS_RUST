///<h2> Collezioni disponibili </h2>
/// <ol style='font-family: Consolas'>
///     <li> Vec<T> </li>
///     <li> LinkedList<T> </li>
///     <li> HashMap<K,V> </li>
///     <li> BTreeMap<K,V> </li>
///     <li> HashSet<K,V> </li>
///     <li> BTreeSet<K,V> </li>
///     <li> BinaryHeap<T> </li>
///     <li> VecDeque<T> </li>
/// </ol>
/// Prova
pub mod collection{
    use std::hash::Hash;

    pub struct Item<T>{
        info: T
    }

    impl<T> Item<T>{
        pub fn new(info:  T)->Self{
            Self{info}
        }
    }

    #[derive(Ord, PartialOrd,PartialEq, Eq)]
    pub struct Key<T: Ord + Hash>{
        chiave: T
    }

    impl<T> Key<T>{
        pub fn new(chiave:T)->Self{
            Self{chiave}
        }
    }
}