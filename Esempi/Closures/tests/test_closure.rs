use Closures::closures::Cagata;

//Test su chiusura
#[test]
pub fn trivial_test(){
    let mut s=String::from("Ciao");

    assert_eq!(Cagata::funz( || s), "Ciao");
}

//Test su metodi map() e  unwrap() su Result<> (gestione errori)
#[test]
pub fn test_error(){
    let mut s=Cagata::prova_err(1).
        map(|value|value+1);

    assert_eq!(s.unwrap(), 2);

    assert_eq!(Cagata::prova_err(0), Err(String::from("Errore!")));
}