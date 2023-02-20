# Strings / Cadenas

en rust existen el tipo `String` y los segmentos `&str` ambos estan codificados en UTF-8. las cadenos se comportan similar a los vectores a la hora de su creacion

```rust
// asi iniciamos una cadena vacia
let mut cad1 = String::new();
// inicializando de otro modo
let cad2 = "contenido Inicial".to_string();
// con el clasico string::from("...")
let cad3 = String::from("Hola Mundo");
// es otro idioma u otros caracteres
let cad4 = String::from("🙋‍♂️🌎");// note: para los emoticones se presiona la teclas windows + .

```

Para **concatenar** es bastante simple

```rust

let mut cad1 = String::from("Hola Mundo");
// es otro idioma u otros caracteres
let cad2 = String::from("🙋‍♂️🌎"); //==> como este elemento no es mutable siempre se pasa su asignacion en memoria.


// del modo clasico
let concat0 = cad1 + &cad2;
// utilizando la instruccion format!
let concat1 = format!("{} {}", cad1, cad2); //==> esta es la forma recomendada
// otra forma es con el push str
cad1.push(&cad2);


```

un detallle importante es que no pordemos acceder a sus caracteres de manera individual

```rust

let cadena = String::from("Hola");
let first_letter = cadena[0];// esto genera un error

```

pero en contra parte si la podemos iterar
```rust

let cadena = String::from("Hola");

// para obtener la letra de cada pocicion
for c in cadena.chars()
{
    println!("{}",c);
}

// para obtener su valor en bytes
for b in cadena.bytes()
{
    println!("{}",b);
}


```

*****
[Regresar](./Readme.md)