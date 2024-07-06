use tde_looper::looper::Looper;
use tde_looper::looper::Message;
#[test]
fn test_looper(){
    let msg = Message::new(64);

    let looper = Looper::<Message<i32>>::new(|msg:Message<i32>|{
        println!("Received message!");
        println!("Message content: {:?}",msg.get_content());
    }, ||{
        println!("Il looper sta per terminare, Ciao!!");
    });

    looper.send(msg);

    let c=looper.get_ricevuti();

    assert_eq!(c.len(), vec![Message::new(64)].len());
}