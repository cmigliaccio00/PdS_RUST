># How to... test `Into`/ `TryInto`

### Come far funzionare i test

Le due *implementazioni* dei tratti seguenti: 

``` Rust
impl Into<f64> for ComplexNumber{
        fn into(self) -> f64{
            if self.Imag != 0.0{
                panic!("Impossible to convert in real: \
                imaginary part it's different than 0");
            }
            self.Real
        }
    }
```

```rust
impl TryInto<f64> for ComplexNumber{
        type Error=String;
        fn try_into(self)->Result<f64,Self::Error>{
            if self.Imag != 0.0{
                Err("Errore".to_string())
            }
            else{
                Ok(self.Real)
            }
        }
    }
```
___...vanno in conflitto___. Per far funzionare i relativi test:

## Per **`TryInto`**
1. Commentare l'**implementazione** di `Into` 
2. Commentare i **test** di `Into`:
    * ``pub fn test_convert_into_real()``
    * `pub fn test_panic_when_impossible_to_convert_to_real()`

## Per `Into`
1. Commentare l'**implementazione** di `TryInto`
2. Commentare il **test** di `TryInto`:
    * `pub fn test_try_into_f64()`

Per quanto riguarda invece il tratto `Into<ComplexNumber>  for f64` non ci sono particolari problemi, 
quindi possono essere runnati senza particolari problemi
insieme agli altri.
