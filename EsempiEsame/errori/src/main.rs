use thiserror::Error;

#[derive(Debug, Error)]
#[error("Problema con il file: {0}")]       //Campo 0 dove si trova la descrizione di questa cosa
struct FileError(&'static str);

#[derive(Debug, Error)]
#[error("Problema con il content: {code}")]     //In questo modo sto implementando il tratto Display
struct ContentError{
    content: String,
    code: i32
}

//Per agevolare la propagazione di errori eterogenei
#[derive(Error, Debug)]
#[error("Errore di elaborazione")]
enum ProcessingError{
    File(#[from] FileError),
    Content(#[from] ContentError)
}

fn leggi_file(filename: &str)->Result<String, FileError>{
    if filename.len() < 3  {
        return Err(FileError("Nome troppo corto"));
    }
    else if filename.len() > 5{
        return Err(FileError("Nome troppo lungo"));
    }
    return Ok("Contenuto del file".to_string());
}

fn elabora_contenuto(content: String)->Result<i32, ContentError>{
    if content.len() > 3{
        return Err(ContentError{content: content.clone(),
            code:1});
    }
    return Ok(1);
}


/* N O T A  U T I L E.
Per poter utilizzare la sintassi '?' per snellire la sintassi e rendere il mio codice
più leggibile, bisogna che il tipo di errore ritornato dalla funzione dopo cui metto '?'
e quello della funzione chiamante sia lo stesso.
Più chiaramente, dobbiamo essere in una situazione del genere:

//---------------funzione che genera l'errore--------------
fn funzione_errore()->Result<T, MyErr>;
//----------------funzione chiamante-----------------------
fn funzione_chiamante(...)->Result<U, MyErr>{
    let v:T = funzione_errore(...)?;
    let a:U = elabora(v);
    Ok(a);
}
Si noti che l'importante è che MyErr sia uguale da una parte e dall'altra
sul tipo di Result non c'è nessun tipo di restrizione!

Nel nostro caso anche se gli errori sono eterogenei ci va bene! Questo perché...
la funzione fn leggi_file() può ritornare FileError, mentre fn contenuto_file()
ritorna potenzialmente un ContentError, entrambi questi tipi di errori però possono
essere convertiti in errori di tipo ProcessingError ricorrendo al tratto From, tale
tratto (dal tipo più specifico al tipo più generico viene automaticamente derivato
grazie alla notazione: Variante(#[from] TipoErrore) */
fn combina_azioni(file_name: &str)->Result<i32, ProcessingError>{
    let c=leggi_file(file_name)?;
    let r = elabora_contenuto(c)?;
    Ok(r)
}


fn main() {
    //Disambiguazione del risultato
    match combina_azioni("abcd"){
        Ok(content)=>
            println!("Il  file contiene {}", content),
        Err(err)=>
            println!("Errore=> {:?}", err),
    }

    //Morale: ho combinato errori molto diversi, ciascuno di questi implementa il tratto Error
    //tramite il crate thiserror, dovendo creare altre funzioni ciascuna delle quali fallisce in
    //modi distinti compatto questi errori in una enum e lascio ancora una volta al crate thiserror
    //per generare i metodi di Error, posso inoltre permettere di estendere un errore specifico
    //a un errore più generale (quello descritto dalla struct)
    //In ciascuna classe di errore posso metter informazioni di qualche genere per gestire
    //il recovery delle informazioni
}
