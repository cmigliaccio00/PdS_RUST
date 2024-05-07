// find all subsequences of seq in s and return a vector of tuples containing the start position
// and the found subsequences as string slices
// ignore overlaps: if a subsequence is found, the search must continue from the next character

// missing lifetimes: the result string slices depend only from one input parameter, which one?

// suggestion: write a function find_sub(&str, &str) -> Option<(usize, &str)> 1
// that finds the first subsequence in a string, you can use it in all the following functions


//token A1-1
fn find_sub<'a> (s: &'a str, token: &str) -> Option<(usize, &'a str)>{
    //Segmentazione del pattern
    let car=&token[0..1].as_bytes()[0];
    let min=&token[1..2].as_bytes()[0] - ('0' as u8);
    let max=&token[3..4].as_bytes()[0] - ('0' as u8);

    //-------------------------------------------------------
    let mut find=false;         //Hai trovato una sotto sequenza?
    let mut conta_char:usize =0;      //contatore caratteri sotto sequenza
    let mut pos=1000;

    //Per ogni carattere della sotto sequenza
    for i in 0..s.len(){
        conta_char=0;

        //Aggancio il primo carattere pari a car
        if(s.as_bytes()[i] == *car){
            conta_char+=1;  //conto il carattere
            pos=i;
            let mut j=i+1;

            //cerco il minimo numero di caratteri
            while j<s.len() && s.as_bytes()[j] == *car && conta_char<=min as usize {
                conta_char += 1;
                j += 1;
            }
            //Provo ad andare avanti
            while j<s.len() && s.as_bytes()[j] == *car && conta_char<max as usize{
                conta_char+=1;
                j+=1;
            }

            if conta_char>=min as usize && conta_char<=max as usize { find=true;}
        }
        if find { break; }
    }

    if(find){  let tup=(pos, &s[pos..pos+conta_char]);
        Some(tup)
    }
    else { None }
}
fn subsequences1<'a> (s: &'a str, seq: &str) -> Vec<(usize, &'a str)> {
    let mut tokens:Vec<&str> = Vec::default();
    let mut ris:Vec<(usize, &'a str)> = Vec::default();
    tokens=seq.split(",").collect();        //Costruisco un vettore di token

    let mut start=0;
    let mut full=true;

    let mut next_pos=0;
    while next_pos < s.len(){
        let mut parziale=String::default();
        full=true;

        //Per ogni token del tipo '<Lettera><Minimo>-<Massimo>'
        for i in 0..tokens.len(){
            let s = find_sub(&s[next_pos..],tokens[i]);

            if (s.is_some()){
                if(i==0){
                    start=next_pos+s.unwrap().0;
                }
                else if(s.unwrap().0!=0) {
                    full = false;         //Trovato ma non consecutivamente
                    break;
                }
                parziale.push_str(s.unwrap().1);
                next_pos = next_pos + s.unwrap().0 + s.unwrap().1.len();
            }
            else{
                full=false;
                break;
            };
        }

        //Hai trovato consecutivamente tutti i pattern richiesti?
        if full{
            //println!("Stringa parziale {}", parziale);
            ris.push((start , &s[start..start+parziale.len()]));
        }
        else{   next_pos+=1;    }
    }
    ris
}

fn find_sub_1<'a> (s: &'a str, seq: &str) -> Option<(usize, &'a str)>{
    let mut tokens:Vec<&str> = Vec::default();
    let mut ris:Vec<(usize, &'a str)> = Vec::default();
    tokens=seq.split(",").collect();        //Costruisco un vettore di token

    let mut start=0;
    let mut full=true;

    let mut next_pos=0;
    while next_pos < s.len(){
        let mut parziale=String::default();
        full=true;

        //Per ogni token del tipo '<Lettera><Minimo>-<Massimo>'
        for i in 0..tokens.len(){
            let s = find_sub(&s[next_pos..],tokens[i]);

            if (s.is_some()){
                if(i==0){
                    start=next_pos+s.unwrap().0;
                }
                else if(s.unwrap().0!=0) {
                    full = false;         //Trovato ma non consecutivamente
                    break;
                }
                parziale.push_str(s.unwrap().1);
                next_pos = next_pos + s.unwrap().0 + s.unwrap().1.len();
            }
            else{
                full=false;
                break;
            };
        }

        //Hai trovato consecutivamente tutti i pattern richiesti?
        if full{
            //println!("Stringa parziale {}", parziale);
            return Some ((start , &s[start..start+parziale.len()]));
        }
        else{   next_pos+=1;    }
    }
    None
}



pub fn demo1() {
    let a = "ACCTAGGCCTTTCTAGCACCCCCCCCCCCCCCCTTTAAAAGGGCCC".to_string();
    let seq = "C1-4,T1-5,A1-4,G1-3,C1-3";

    for (off, sub) in subsequences1(&a, seq) {
        println!("Found subsequence at position {}: {}", off, sub);
    }
}


// Now we want to find different subsequences at the same time, seq is a vector of string slices with many subsequence to search
// For each subsequence find all the matches and to the results (there may be overlaps, ignore them), but in this way you can reuse the previous solution
// The result will contain: the start position in s, the found subsequence as string slice and the mached subsequence in seq
// Now the string slices in the rsult depend from two input parameters, which ones?
fn subsequences2<'a, 'b>(s: &'a str, seq: &[&'b str]) -> Vec<(usize, &'a str, &'b str)> {
    let mut ris:Vec<(usize, &str, &str)> = Vec::default();
    let mut intermediate:Vec<(usize,&str)> = Vec::default();

    for i in 0..seq.len(){
        let mut matcher=seq[i];

        intermediate=subsequences1(s, matcher);

        for (off, sub) in intermediate{
            let tupla=(off, sub, matcher);
            ris.push(tupla);
        }
    }
    ris
}

pub fn demo2() {
    let a = "AACGGTTAACC".to_string();
    let seqs = ["A1-1,C2-4", "G1-1,T2-4"];

    for (off, matched, sub) in subsequences2(&a, &seqs) {
        println!("Found subsequence {} at position {}: {}", matched, off, sub);
    }
}


// Now we want to do some DNA editing! Therefore we receive a mutable string and we'd like to return a vector of mutable string slices
// Follow this steps:
// 1. adjust the lifetimes without any implementation yet: does it compile?
// 2. try to implement the function: does it compile?
// 3. if it doesn't compile, try to understand why from the compiler errors and draw all the necessary lifetimes
// 4. Spoiler: basically it's not possibile to return more then one mutable reference to the same data
// 5. Try this workaround: return a vector of indexes (first solution) and let the caller extract the mutable references
// 7. (later in the course you will learn about smart pointers, which can be used to solve this kind of problems in a more elegant way)
fn subsequences3 (s: &str, seq: &str) -> Vec<(usize, usize)> {
    let mut tokens:Vec<&str> = Vec::default();
    let mut ris:Vec<(usize, usize)> = Vec::default();
    tokens=seq.split(",").collect();        //Costruisco un vettore di token

    let mut start=0;
    let mut full=true;

    let mut next_pos=0;
    while next_pos < s.len(){
        let mut parziale=String::default();
        full=true;

        //Per ogni token del tipo '<Lettera><Minimo>-<Massimo>'
        for i in 0..tokens.len(){
            let s = find_sub(&s[next_pos..],tokens[i]);

            if (s.is_some()){
                if(i==0){
                    start=next_pos+s.unwrap().0;
                }
                else if(s.unwrap().0!=0) {
                    full = false;         //Trovato ma non consecutivamente
                    break;
                }
                parziale.push_str(s.unwrap().1);
                next_pos = next_pos + s.unwrap().0 + s.unwrap().1.len();
            }
            else{
                full=false;
                break;
            };
        }

        //Hai trovato consecutivamente tutti i pattern richiesti?
        if full{
            //println!("Stringa parziale {}", parziale);
            ris.push((start , parziale.len()));
        }
        else{   next_pos+=1;    }
    }
    ris
}

pub fn demo3() {
    let mut a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    for (off, len) in subsequences3(&a, seq) {
        let mut sub =  &mut a[off..off+len];

        println!("Found subsequence at position {}: {}", off, sub);
    }
}


// DNA strings may be very long and we can get a lot of matches.
// Therefore we want to process a subsequence as soon as we find it, without storing it in a vector
// A solution is to pass a closure to the function, which will be called for each match
// do you need to put lifetime annotations in the closure? why?
fn subsequence4<F>(s: &str, seq: &str, closure: F)
    where F: Fn(usize, &str)
{
    let ris = subsequences1(s, seq);

    for el in ris{
        closure(el.0, el.1);
    }
}

pub fn demo4() {
    let a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    subsequence4(&a, seq, |pos, sub| {
        println!("Found subsequence at position {}: {}", pos, sub);
    });
}


// Now let's define a struct SimpleDNAIter (add the required lifetimes), memorizing a DNA sequence and the subsequence to search
// Then we add a next() method to the struct, which will return the next subsequence found in the DNA sequence after each call
// The result of next() is a tuple, but it's wrapped in an Option, because a call to next() may find no more subsequences in the DNA sequence
// In order to implement it, you may add any other attribute to the struct (remember: the struct is stateful and after each call to next() you must start from the last position found)
// The struct may be used as shown in the demo_SimpleDNAIter() function
// This approach is similar to the previous one, but it's more flexible and it can be used in more complex scenarios. For example you may interrupt it
// at any time and resume it later

struct SimpleDNAIter<'a, 'b>
{
    s: &'a str,
    seq: &'b str,
    items: Vec<(usize, &'a str)>,
    stato: usize
}

impl<'a, 'b> SimpleDNAIter<'a, 'b>
{
    pub fn new(s: &'a str, seq: &'b str) -> Self {
        //Posso farlo perché le variabili hanno
        //lo stesso nome dei campi
        let items=subsequences1(s,seq);
        let stato=0;
        SimpleDNAIter {s,seq, items,stato}
    }

    pub fn add_stato(&mut self){
        self.stato = self.stato+1;
    }

    //return the next subsequence found in the DNA sequence after each call
    pub fn next(&mut self) -> Option<(usize, &str)> {
        if (self.items.len() == 0 ||
            self.stato>=self.items.len()) {
            None
        } else {
           let ris= Some(self.items[self.stato]);
            self.add_stato();
            ris
        }
    }
}

pub fn demo_SimpleDNAIter() {
    let mut dna_iter = SimpleDNAIter::new("ACGTACGTACGTACGT", "A1-1,C1-1");

    while let Some((pos, subseq)) = dna_iter.next() {
        println!("Found subsequence at position {}: {}", pos, subseq);
        // we can break and stop if we have found what we were looking for
    }
}


// finally we want to implement a real iterator, so that it can be used in a for loop
// and it may be combined we all the most common iterator methods
// The struct DNAIter is already defined, you have to implement the Iterator trait for it and add lifetimes
struct DNAIter<'a, 'b>{
    s: &'a str,
    seq: &'b str,
    items: Vec< (usize, &'a str)>,
    stato: usize
}

impl<'a, 'b> DNAIter<'a, 'b> {
    pub fn new(s: &'a str, seq: &'b str) -> DNAIter<'a, 'b> {
        let items=subsequences1(s, seq);
        let stato=0;
        DNAIter {s, seq, items, stato}
    }

    fn update_stato(&mut self){
        self.stato +=1;
    }
}

impl<'a, 'b> Iterator for DNAIter<'a, 'b>{
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if(self.items.len()==0 || self.stato>=self.items.len()){
            None
        }
        else{
            let ris = Some(self.items[self.stato]);
            self.update_stato();
            ris
        }
    }
}

pub fn demo_dna_iter() {
    let dna_iter = DNAIter::new("ACGTACGTAAACCCGTACGT", "A1-3,C1-2");

    // now you can combine it with all the iterator modifiers!!!
    dna_iter
        .filter(|(pos, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}


// now let's return an iterator without defining a struct, just using a closure
// the std lib of rust support you with the std::from_fn() function
// we supply a skeleton implementation, you have to fill the closure
fn subsequence5_iter<'a>(s: &'a str, seq: &'a str) -> impl Iterator<Item = (usize, &'a str)> {
    let mut pos = 0;
    // and any other necessary variable to remember the state
    std::iter::from_fn(move || {
        let ris = subsequences1(s,seq).iter();
        if let Some(k) = find_sub_1(&s[pos..], seq) {
            let mut ris=Some(k).unwrap();
            ris.0+=pos;
            pos=pos+k.1.len();
            Some(ris)
        } else {
            None
        }
    })
}

pub fn demo_dna_iter2() {
    subsequence5_iter("ACGTACGTAAACCGTACGTAAACCGTACGT", "A1-3,C1-2")
        .filter(|(pos, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}