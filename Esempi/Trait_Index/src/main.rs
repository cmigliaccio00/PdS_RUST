use std::ops::Index;

#[derive(Debug)]
struct Esame{
    nome_corso: String,
    voto: i32
}

struct Studente{
    matricola: u32,
    esami: Vec<Esame>
}

impl Esame{
    fn new()->Self{
        Esame{nome_corso: String::from(""), voto:0}
    }

    fn from_data(nome: &str, voto_es: i32)->Self{
        Esame{nome_corso: String::from(nome), voto: voto_es}
    }
}

impl Studente{
    fn new(mat: u32) -> Self{
        Studente{matricola: mat, esami: Vec::new()}
    }

    fn add_esame(&mut self, e: Esame){
        self.esami.push(e);
    }

    fn voto_from_esame(&self, nome: String)->&i32{

        for i in 0..self.esami.len(){
            if nome.eq(self.esami.index(i).nome_corso.as_str()){
                return &self.esami[i].voto;
            }
        }
        return &self.esami[0].voto
    }

    fn length(&self)->usize{
        self.esami.len()
    }
}

//Implementazione del tratto Index
impl Index<&str> for Studente{
    type Output=i32;

    //Qui devo tornare un indirizzo!
    fn index(&self, index: &str)->&Self::Output{
        self.voto_from_esame(String::from(index))
    }
}

fn main() {
    /*
    let mut stud=Studente::new(332937);

    stud.add_esame(Esame::new());
    stud.add_esame(Esame::from_data("Calcolatori_Elettronici", 27));
    stud.add_esame(Esame::from_data("Controlli Automatici" , 30));
    stud.add_esame(Esame::from_data("Chimica" , 30));
    stud.add_esame(Esame::from_data("Architetture dei sistemi di elaborazione", 30));

    println!("Esame: {:?}", stud["Controlli Automatici"]);*/

    let a=5;
    let b=6;

    let c=a+b;

    println!("{:?}", a);

}
