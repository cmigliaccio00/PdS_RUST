/**
Modulo 'polimorfismo'

Copyright(C) 2024 by Carlo Migliaccio
Creative commons licence.

*/

pub mod poli{
    //Imponiamo che l'errore implementato dipenda da qualsiasi tipo
    //purché sia copiabile (where T: Copy)
    pub struct Errore<T>
        where T: Copy{
        campo : T,
        descrivi: String
    }

    impl<T: Copy> Errore<T> {
        pub fn new(campo: T)->Self{
            Self{campo, descrivi: String::new()}
        }
        pub fn get(&self) -> T {
            self.campo
        }
        pub fn set_descr(&mut self, d: String){
            self.descrivi = d;
        }
        pub fn get_descr(&self)->&str{
            &self.descrivi
        }
    }
}

