/******************************************************************
Esempi Esame: Smart Pointers

Copyright (C) 2024 - Carlo Migliaccio
All rights are reserved.

                        Last modify: 01.07.2024
******************************************************************/
use std::rc::Rc;
use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct Studente{
    nome: String,
    eta: i32,
    exp: Cell<i32>,
    other: RefCell<i32>
}

impl Studente{
    fn new() ->Self{
        Studente{nome: String::new(),
            eta: 0,
            exp:Cell::new((0)),
            other: RefCell::new(0)}
    }
    fn with_attributes(nome: String, eta: i32)->Self{
        Self{nome,eta,
            exp:Cell::new(0),
            other: RefCell::new(0)}
    }
    fn set_nome(&mut self, nome: String)->&Self{
        self.nome = nome;
        self
    }
    fn set_eta(&mut self, eta: i32)->&Self {
        self.eta = eta;
        self
    }

    fn println_stud(&self){
        println!("Nome: {}, Eta: {}", self.nome, self.eta);
    }

    fn get_nome(&self)->&str    {       &self.nome      }
    fn get_eta(&self)->i32      {    self.eta           }
}

fn main() {
    let mut my_box = Box::new(Studente::new());         //Box normale
    let my_rc = Rc::new(Studente::with_attributes(
                        "Antonio".to_string(),
                        13));

    //Modifico la struttura
    my_box.set_eta(15);
    my_box.set_nome("Carlo".to_string());

    let nome = my_rc.get_nome();
    let eta = my_rc.get_eta();

    println!("{:?}, {:?}", nome, eta);

    my_box.println_stud();
    my_rc.println_stud();

    let clone1 = my_rc.clone();
    println!("Strong count {}", Rc::<Studente>::strong_count(&my_rc));

    //Questo riferimento non tiene in vita il dato
    let weak1 = my_rc.clone();
    let weak1= Rc::<Studente>::downgrade(&weak1);

    println!("Weak count {}", Rc::<Studente>::weak_count(&my_rc));

    //Sfrutto la interior mutability: nota che in quanto tale un Rc
    //non potrebbe essere modificato
    my_rc.exp.set(5);

    //Prendo un riferimento mutabile alla RefCell contenuta nella struttura
    let mut reftoRefCell = my_rc.other.borrow_mut();
    *reftoRefCell=5;
    drop(reftoRefCell);

    let borrow = my_rc.other.try_borrow().unwrap();
    println!("Value of the refcell: {}", borrow);

    println!("Mia struttura: {:?}", my_rc);

    //Puntatori senza possesso
    let mut mia_Struct = Studente::with_attributes("Lia".to_string(), 58);


    let refStruct = &mia_Struct;

    //mia_Struct.nome = "Carlo".to_string();

    //La presenza di questa riga porterebbe a un errore di compilazione!
    //in quanto è in giro un riferimento (immutabile) al dato. In altre
    //parole, poiché il dato è stato prestato (borrowed), NON può essere
    //modificato finché c'è anche solo un riferimento mutabile in giro

    //Se ho un RefMut in giro ho da adottare la poltica single writer

    println!("{:?}", refStruct);

    let mut refS = &mut mia_Struct;
}
