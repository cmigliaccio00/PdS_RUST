use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct People{
    name: String,
    cognome: String,
    eta: usize
}

fn main() {

}
