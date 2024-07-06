use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::path::PathBuf;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Tipo{
    field1:i32,
    field2:i32,
}

impl Tipo{
    fn with_attributes(f1:i32, f2:i32)->Self{
        Self{field1:f1, field2:f2}
    }

    fn serializza(&self)->String{
        serde_json::to_string(self).unwrap()
    }

    fn deserializza(stringa: String)->Self{
        let a:Tipo = serde_json::from_str(&stringa).unwrap();
        a
    }

    fn salvaFile(&mut self, Nome: PathBuf)->Result<i32, Error>{
        let mut file = File::open(Nome)?;
        file
                .write_all(self.serializza()
                .as_bytes())
                .unwrap();
        Ok(1)
    }
}

fn main() {
    let nome_file: &'static str = "Serial.txt";

   for i in 1..20{
       let mut var = Tipo::with_attributes(i,i+1);
       var.salvaFile(PathBuf::from_str(nome_file).unwrap());
   }
}
