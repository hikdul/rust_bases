# Slice

es un tipo de dato que no tiene propiedad y que nos permite hacer referencia a una secuencia contigua de elementos de una coleccion en lugar de a toda la coleccion

```rust

    fn main()
    {
        let mut cad = String::from("hola mundo");    
        let primera = first_word(&cad);
        println!("primera palabra: {}", primera);
    }

    fn first_word(cad: &str) -> str
    {
        for( i, &item) in bytes.iter().enumerate()
        {
            if item == b' '
            {
                return &cad[0..i];
            }
        }
        &cadena[..]
    }
```

*****
[Regresar](./Readme.md)