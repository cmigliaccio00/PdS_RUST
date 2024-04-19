use clap::builder::NonEmptyStringValueParser;
//Uso di clap
use clap::Parser;

//Parametri di clap
#[derive(Parser, Debug)]
struct Args {
    // input string
    //#[arg(short, long)]
    slug_in: String,
    //#[arg(default_value="0")]
    repeat: Option<i32>,
    //#[arg(default_value = "true")]
    verbose: Option<bool>,

    #[arg(action = clap::ArgAction::Count)]
    count: u8
}

//----------------------TEST--------------------------
#[cfg(test)]
mod tests {
    use super::*;

    //Test sulla funzione conv
    #[test]
    fn lettera_accentata() {
        // valore = preparazione test
        assert_eq!(conv('è'), 'e');
    }

    #[test]
    fn lettera_non_accentata(){
        assert_eq!(conv('ì'), 'i');
    }

    #[test]
    fn lettera_no_admit(){
        assert_eq!(conv('#'), '-');
    }

    #[test]
    fn lettera_acc_no_admit(){
        assert_eq!(conv('ῶ'), '-');
    }

    //Test sulla funzione slugify
    #[test]
    fn stringa_spazio(){
        assert_eq!(slugify("carlo migliaccio 332937"),
                   String::from("carlo-migliaccio-332937"));
    }

    #[test]
    fn stringa_vuota(){
        assert_eq!(slugify(""), String::from(""));
    }

    #[test]
    fn stringa_accentata(){
        assert_eq!(slugify("carloèèèèèèèmigliaccio"), String::from("carloeeeeeeemigliaccio"));
    }

    #[test]
    fn stringa_piu_spazi(){
        assert_eq!(slugify("carlo   migliaccio"), String::from("Carlo---Migliaccio"));
    }

    #[test]
    fn stringa_non_validi_cons(){
        assert_eq!(slugify("#######carlo"), "-------carlo");
    }

    #[test]
    fn stringa_non_validi(){
        assert_eq!(slugify("#######"), "-------");
    }

}

//--------------------FUNZIONI----------------------------------
/***********************************************************
Le stringhe in Rust sono di due tipi:
-Mutabili (espresse con &str)
-Immutabili(indicate con String e simili ai Vec<T>)
**********************************************************/
fn slugify(s:&str) -> String{
    let mut res=String::new();
    let mut copy=String::from(s);       //Copio la stringa mutabile

    //Tutto in minuscolo
    copy=copy.to_lowercase();
    let mut i=0;

    for car  in copy.chars(){
        res.push(  conv( car) );
    }
    res
}

fn conv (c:char)->char{
    //Attenzione! Alcuni caratteri sono rappresentati su piu' di un byte
    const SUBS_I : &str =
        "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóoeøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str =
        "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzzz";

    //Creo dalle stringhe immutabili le rispettive stringhe mutabili
    let mut subs_i = String::from(SUBS_I);
    let mut subs_o=String::from(SUBS_O);

    //Cerco tra le lettere brutte
    let mut conta=0;
    for car  in subs_i.chars(){
        if car==c{ return char::from(subs_o.as_bytes()[conta]); }
        conta=conta+1;
    }

    if ((c>='a' && c<='z' ) || (c>='0' && c<='9'))  { return c;     }

    //Nessuna corrispondenza trovata (ritorno '-'
    '-'
}

fn main() {
    let S="Ciao Carlo";

    println!("Prima: {}", S);
    println!("Dopo:  {}", slugify(S));
}


