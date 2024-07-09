use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, long_about=None)]
struct Argomenti{
    ///Nome della persona
    #[arg(short, long)]
    nome: String,
    ///Cognome della persona
    #[arg(short, long)]
    cognome: String
}

fn main() {
    let a = Argomenti::parse();
    println!("Ciao ti chiami {} {}", a.nome, a.cognome);
}
