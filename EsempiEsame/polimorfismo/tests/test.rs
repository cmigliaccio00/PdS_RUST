use polimorfismo::poli::Errore;

#[test]
fn try_get(){
    let err = Errore::new(5);
    assert_eq!(err.get(), 5);
}

#[test]
fn try_description(){
    let mut err = Errore::new(4);
    err.set_descr("Errore generico".to_string());
    assert_eq!(err.get_descr(), "Errore generico".to_string());
}




