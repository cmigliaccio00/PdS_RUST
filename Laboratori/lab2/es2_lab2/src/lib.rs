#[allow(non_snake_case)]
pub mod solution{
    use std::ops::{Add, AddAssign};
    use std::process::Output;

    #[derive(Copy, Clone)]
    pub struct ComplexNumber{
        Real: f64,
        Imag: f64
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

}