pub mod solution{
    use std::collections::VecDeque;
    use std::ops::Deref;

    pub struct CircularBuffer<T> {
        cap: usize,
        array: Vec<T>,
        head: usize,
        tail: usize,
        size: usize
    }

    impl<T> CircularBuffer<T>
        where T: Default + Copy + ?Sized
    {
        //Costruttore
        pub fn new(capacity: usize) -> Self{
            let mut C:CircularBuffer<T>=CircularBuffer{array: Vec::with_capacity(capacity),
                head: 0,
                tail:0,
                size: 0,
                cap: capacity};
            for _ in 0..capacity{
                C.array.push(T::default());
            }
            C
        }

        pub fn write(&mut self, item: T) -> Result<String,String>{
            //Controllo di coda piena
            if  self.size!=self.cap{

                self.array[self.tail]=item;
                self.tail = self.tail+1;        //Conto l'elemento appena aggiunto
                if(self.tail==self.cap){
                    self.tail=0;
                }
                self.size=self.size+1;
                Ok(String::from("OK"))
            }
            else{
                Err(String::from("Buffer Pieno"))
            }
        }

        pub fn read(&mut self) -> Option<T> {
            //Controllo di coda vuota
            if (self.head==self.tail && self.size==0){
                None
            }
            else{
                let ret=self.array[self.head];
                //Lascio il valore di default
                self.array[self.head]=T::default();

                self.head = self.head+1;
                if (self.head==self.cap){
                    self.head=0;
                }
                self.size=self.size-1;
                Some(ret)
            }
        }

        pub fn clear(&mut self){
            self.size=0;
            self.head=0;
            self.tail=0;
        }

        pub fn size(&self) -> usize{
            self.tail
        }

        pub fn make_contiguos(&mut self) {
            if self.tail < self.head{
                println!("entro qui");
                let size=(self.cap-self.head);
                let Tmp=self.tail;

                for i in self.head..self.cap{
                    self.array[self.tail]=self.array[i];
                    self.array[i]=T::default();
                    self.tail = self.tail + 1;
                }

                //La testa si sposta dove prima era la coda
                self.head=Tmp;
            }
        }

        pub fn get_head(&self) -> usize{
            self.head
        }

        pub fn get_tail(&self) -> usize{
            self.tail
        }

        pub fn get_buf(&self) -> &Vec<T> {
            &self.array
        }

    }
}