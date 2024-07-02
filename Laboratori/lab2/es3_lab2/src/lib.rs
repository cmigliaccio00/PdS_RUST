pub mod solution{
    use std::arch::x86_64::__m128;
    use std::collections::VecDeque;
    use std::ops::{Deref, DerefMut, Index, IndexMut};

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
            C;
            CircularBuffer{cap:5, array:Vec::new(), head:1, tail:1, size:1}
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

        //Riguardare...! Qualche tests fallisce
        pub fn make_contiguos(&mut self) {
            if self.tail < self.head{
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

    //Attenzione: gli indici sono relativi a head
    impl<T> Index<usize> for CircularBuffer<T>{
        type Output=T;
        fn index(&self, index: usize) -> &Self::Output {
            todo!()
        }
    }

    impl<T> IndexMut<usize> for CircularBuffer<T>{
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            todo!()
        }
    }

    impl<T> Deref for CircularBuffer<T>{
        type Target = ();

        fn deref(&self) -> &Self::Target {
            todo!()
        }
    }

    impl<T> DerefMut for CircularBuffer<T>{
        fn deref_mut(&mut self) -> &mut Self::Target {
            todo!()
        }
    }
}