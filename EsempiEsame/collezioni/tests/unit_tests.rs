use std::collections::BTreeMap;
use collezioni::collection::Item;
use collezioni::collection::Key;
#[test]
fn test_vec(){
    let mut a  = Vec::<Item<i32>>::new();

    a.push(Item::new(1));
    a.push(Item::new(2));

    assert_eq!(a.len(),2);
}

#[test]
fn test_linked_list(){
    todo!()
}

#[test]
fn test_btreemap(){
    let mut  a = BTreeMap::<Key<i32>,Item<String> >::new();

    a.insert(
        Key::new(272489),
        Item::new("Carlo Migliaccio".to_string())
    );
    todo!()
}

