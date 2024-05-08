use crate::es0301::{demo1, demo2, demo3, demo4, demo_SimpleDNAIter, demo_dna_iter, demo_dna_iter2};
use crate::es0302::{demo};

mod es0301;
mod es0302;

fn main() {

    let What=2;

    if What==1{
        //Esercizio 1 - Lifetime, uso di Iteratori
        println!("demo1");
        demo1();
        println!("demo2");
        demo2();
        println!("demo3");
        demo3();
        println!("demo4");
        demo4();
        println!("demo_SimpleDNAIter");
        demo_SimpleDNAIter();
        println!("demo_dna_iter");
        demo_dna_iter();
        println!("demo_dna_iter2");
        demo_dna_iter2();
    }
    else{
        //Esercizio 2
        demo();
    }
}