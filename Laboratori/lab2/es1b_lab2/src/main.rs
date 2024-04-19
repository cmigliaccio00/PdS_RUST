fn slugify(s:&str) -> String{
    let mut res=String::new();
    let mut copy=String::from(s);       //Copio la stringa mutabile

    //Tutto in minuscolo
    copy=copy.to_lowercase();
    let mut i=0;

    for car  in copy.chars()    { res.push(  conv( car) ); }
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

    let so=&subs_o;
    //Cerco tra le lettere brutte
    let mut conta=0;
    for car  in subs_i.chars(){
        if car==c{ return char::from(so.as_bytes()[conta]); }
        conta=conta+1;
    }
    if ((c>='a' && c<='z' ) || (c>='0' && c<='9'))  {   return c;     }

    //Nessuna corrispondenza trovata (ritorno '-'
    return '-';
}

//Il tratto non è generico
trait MySlug{
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

//Sia str che String implementano:
// AsRef<str> for String       <mia_String>.as_ref -> &str
// AsRef<str> for &str         <mia_str>.as_ref -> &str

impl<T> MySlug for T
    where T:AsRef<str> {
    fn is_slug(&self) -> bool {
        let slug = self.to_slug();
        return slug.eq(self.as_ref())
    }
    fn to_slug(&self) -> String{
        slugify(self.as_ref())
    }
}

fn main() {
    //Stringhe cambiate (per il momento)
    let s1 = String::from("Ciao Carlo");
    let s2 = "ciao-carlo";

    println!("{}", s1.is_slug());
    println!("{}", s2.is_slug());

    let s3=s1.to_slug();
    let s4=s2.to_slug();

    println!("s3: {} s4: {}", s3, s4);
}

