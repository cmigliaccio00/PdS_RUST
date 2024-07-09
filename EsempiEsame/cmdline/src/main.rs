use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, long_about=None)]
struct Argomenti{
    ///Primo campo della struttura
    #[arg(short, long)]
    campo1: String,
    ///Secondo campo della struttura
    #[arg(short, long)]
    altro: String,
}

fn main() {
    let arg = Argomenti::parse();

    println!("Contenuto degli argomenti: {:?}",arg);
}
