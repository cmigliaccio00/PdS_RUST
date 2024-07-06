use iteratori::it;
use iteratori::it::prova_iteratore;

#[test]
fn test_iterator(){
    let a =  prova_iteratore(vec![1,2,3]);
    assert_eq!(a, vec![2,3,4]);
}