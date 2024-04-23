#[allow(non_snake_case)]

use std::fmt::Display;
use std::collections::VecDeque;

pub mod solution{
    use std::cmp::Ordering;
    use std::collections::VecDeque;
    use std::error::Error;
    use std::fmt;
    use std::fmt::Formatter;
    use std::hash::{Hash, Hasher};
    use std::ops::{Add, AddAssign};
    use std::process::Output;

    #[derive(Copy, Clone, Default, Debug, PartialEq)]
    pub struct ComplexNumber{
        Real: f64,
        Imag: f64
    }

    impl fmt::Display for ComplexNumber{
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f,"{} + j {}", self.Real, self.Imag)
        }
    }


    impl ComplexNumber{
        //Costruttore
        pub fn new(R: f64, I:f64)->Self{
            ComplexNumber{Real:R, Imag:I}
        }

        //Creazione complesso da numero reale
        pub fn from_real(R:f64)->Self{
            ComplexNumber{ Real:R, Imag: 0.0}
        }
        //Conversione in tupla
        pub fn to_tuple(&self) -> (f64,f64){
            (self.Real, self.Imag)
        }

        pub fn print(&self){
            println!("{} + j{}", self.Real, self.Imag);
        }

        pub fn real(&self)->f64{
            self.Real
        }

        pub fn imag(&self)->f64{
            self.Imag
        }

        pub fn norm(&self)->f64{
            ((self.Real*self.Real) + (self.Imag*self.Imag)).sqrt()
        }

    }

    impl Add for ComplexNumber{
        type Output=Self;
        fn add(self, other: Self)->Self{
            ComplexNumber{Real: (self.Real+other.Real), Imag:(self.Imag+other.Imag)}
        }
    }

    impl Add<f64> for ComplexNumber{
        type Output=Self;
        fn add(self, other: f64)->Self{
            ComplexNumber{Real: (self.Real+other), Imag: self.Imag}
        }
    }

    impl AddAssign for ComplexNumber{
        fn add_assign(&mut self, rhs: Self){
            self.Real=self.Real+rhs.Real;
            self.Imag=self.Imag+rhs.Imag;
        }
    }

    impl Add<&ComplexNumber> for ComplexNumber{
        type Output=Self;
        fn add(self, other: &Self)->Self{
            ComplexNumber{Real: self.Real+other.Real, Imag: self.Imag+other.Imag}
        }
    }

    impl Add for &ComplexNumber{
        type Output=ComplexNumber;

        fn add(self, other: Self)->ComplexNumber{
            ComplexNumber{Real: self.Real+other.Real, Imag: self.Imag+other.Imag}
        }
    }


    //Conversioni di tipo
    impl From<f64> for ComplexNumber{
        fn from(num: f64) -> Self{
            ComplexNumber{Real: num, Imag: 0.0}
        }
    }

    impl Into<f64> for ComplexNumber{
        fn into(self) -> f64{
            if self.Imag != 0.0{
                panic!("Impossible to convert in real: \
                imaginary part it's different than 0");
            }
            self.Real
        }
    }



    /*
    impl TryInto<f64> for ComplexNumber{
        type Error=String;
        fn try_into(self)->Result<f64,Self::Error>{
            if self.Imag != 0.0{
                Err(String::from("Errore"))
            }
            else{
                Ok(self.Real)
            }
        }
    }


    impl TryFrom<ComplexNumber> for f64{
        type Error=String;
        fn try_from(value: ComplexNumber) -> Result<Self, Self::Error> {
            if value.Imag != 0.0{
                Err(String::from("Errore di conversione"))
            }
            else {
                Ok(value.Real)
            }
        }
    }
    impl Into<ComplexNumber>  for f64{
        fn into(self) -> ComplexNumber {
            ComplexNumber{Real: self, Imag: 0.0}
        }
    }
    */


    //Per rendere compatibile i numeri complessi con l'ordinamento
    impl PartialOrd for ComplexNumber{
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.norm() > other.norm(){
                Some(Ordering::Less)
            }
            else if self.norm() < other.norm(){
                Some(Ordering::Greater)
            }
            else if self.norm() == other.norm(){
                Some(Ordering::Equal)

            }
            else {
                None
            }
        }

        fn ge(&self, other: &Self) -> bool {
            self.norm() > other.norm() || self.norm().total_cmp(&other.norm())==Ordering::Equal
        }

        fn le(&self, other: &Self) -> bool {
            self.norm() < other.norm() || self.norm().total_cmp(&other.norm())==Ordering::Equal
        }

        fn gt(&self, other: &Self) -> bool {
            self.norm() > other.norm()
        }

        fn lt(&self, other: &Self) -> bool {
            self.norm() < other.norm()
        }
    }

    impl Eq for ComplexNumber{

    }
    impl Ord for ComplexNumber{
        fn cmp(&self, other: &Self) -> Ordering {
           self.norm().total_cmp(&other.norm())
        }
    }

    //Ritorna un ref alla parte reale
    impl AsRef<f64> for ComplexNumber{
        fn as_ref(&self) -> &f64 {
            &self.Real
        }
    }

    //Ritorna un ref mut alla parte reale
    impl AsMut<f64> for ComplexNumber{
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.Real
        }
    }

    impl Hash for ComplexNumber{
                //Va bene così?
        fn hash<H: Hasher>(&self, state: &mut H) {
            (self.Real+self.Imag).to_bits().hash(state)
        }
    }
}