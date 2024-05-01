pub mod closures{

    pub struct Cagata{
        miocampo:i32
    }

    impl Cagata{
        pub fn funz<F>(f: F)->String
            where F: FnOnce()->String
        {
            f()
        }

        pub fn prova_err(num: usize) -> Result<usize, String>{
            if(num==1){
                Ok(num)
            }
            else
            {
                Err(String::from("Errore!"))
            }
        }
    }
}