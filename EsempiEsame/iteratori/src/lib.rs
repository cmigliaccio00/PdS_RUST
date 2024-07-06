pub mod it{
    use std::collections::LinkedList;

    ///<u> Documentazione di esempio </u>
    pub fn prova_iteratore(vet: Vec<i32>) ->Vec<i32>{
        let a = vet
            .iter()                 //Estrae iteratori da elementi collezione
            .map(|&a| a+1)      //Sommo ad ogni elemento 1
            .collect();                                 //Metto questo vettore in una collezione
        a
    }
}
