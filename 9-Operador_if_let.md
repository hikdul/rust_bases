# Operador if let

 el operador if let nos permite combinar if y let pero con menos detalles que cuando se usa el operador match, la gran diferencia se debe a que se manejan valores que coinciden con in patron y se ignoran a resto
 
 ```rust

 fn main(){

    let maximo_configurado = Some(7u8);

    if let some(maximo) = maximo_configurado{
        println!("el maximo configurado es: {}", maximo);
    }

 }
 
 ```
  
*****
[Regresar](./Readme.md)