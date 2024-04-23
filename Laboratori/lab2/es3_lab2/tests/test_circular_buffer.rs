use es3_lab2::solution::CircularBuffer;

#[test]
fn test_inserimento_dimensione(){
    let mut Buffer:CircularBuffer<i32> = CircularBuffer::new(5);

    assert_eq!(Buffer.write(10), Ok(String::from("OK")));
    assert_eq!(Buffer.size(), 1);
}

#[test]
fn test_lettura(){
    let mut  Buffer:CircularBuffer<i32>=CircularBuffer::new(5);

    Buffer.write(4).unwrap();
    let b=Buffer.read().unwrap();
    assert_eq!(b, 4);
}

#[test]
fn test_lettura_n_el(){
    let mut buffer:CircularBuffer<i32>=CircularBuffer::new(3);

    //Inserimento elementi
    buffer.write(1).unwrap();
    buffer.write(2).unwrap();
    buffer.write(3).unwrap();

    //Li provo a leggere e vedo se e' tutto ok
    assert_eq!(buffer.read().unwrap(), 1);
    assert_eq!(buffer.read().unwrap(), 2);
    assert_eq!(buffer.read().unwrap(), 3);
}

#[test]
fn test_leggi_vuoto(){
    let mut buffer:CircularBuffer<i32>=CircularBuffer::new(3);

    assert_eq!(buffer.read(), None);
}

#[test]
fn test_scrivi_pieno(){
    let mut buf:CircularBuffer<i32> = CircularBuffer::new(3);

    buf.write(1).unwrap(); buf.write(2).unwrap(); buf.write(3).unwrap();

    assert_eq!(buf.write(3), Err(String::from("Buffer Pieno")));
    assert_eq!(buf.get_head(), 0);
    assert_eq!(buf.get_tail(), 0);
}

#[test]
fn test_overwrite_pieno(){
    todo!()
}

#[test]
fn test_contiguo() {
    let mut buf: CircularBuffer<i32> = CircularBuffer::new(5);

    buf.write(1);
    buf.write(2);
    buf.write(3);
    buf.write(4);
    buf.write(5);

    buf.read();
    buf.read();
    buf.read();
    buf.write(1);
    buf.make_contiguos();

    let v = buf.get_buf();
    let vet = [1, 4, 5];

    //assert_eq!(v, vet.as_slice());
    //assert_eq!(v, vet.as_slice());

    assert_eq!(buf.get_head(), 0);
    assert_eq!(buf.get_tail(),3);
}






