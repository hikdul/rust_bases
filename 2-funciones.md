# Funciones en rust

Para declarar una funcion de antepone la palabra reservada 'fn'. Para la declaracion de funciones en rust se usa la convencion 'sanke case'.

la forma de crear una funcion es la siguiente

```rust

fn nombre_funcion(...parametros)
{// inicio
    ... cuerpo de la funcion   
}// fin de la funcion

```
_________________

##### ___snake case___
todos los nombres van en minusculas y para su separacion se usa el piso(_) por ejemplo: snake_case

_________________


## Declaraciones y Expreciones 

1. las **expreciones** son aquellas funciones que evaluan una exprecion(rebundante) y devuelven un resultado

```rust
    let i = 10;
    
    // * esta es mi exprecion
    let j = {
        let k = i+1;
        k+1
    };
    println!("el valor de J = {}", j);
```
2. una **declaracion** solo se ejecuta pero no retorna ningun valor

```rust

fn declaracion(i:i32)
{
   println!("el valor ingresado es: {}",i) ;
}
```

## Funciones que devuelven valores

Para que la funcion predefina el tipo de dato se usa una flecha '->' luego de los () de la funcion y alli se indica el tipo de dato para generar el retorno se deja como ultima opcion el no usar ; o si se desea ser explicito se usa la palabra reservada 'return'

```rust
    // ejemplo con return
    fn suma (a:i64, b:i64) -> i64 {
        return a+b;
    }
    // ejemplo sin return
    fn resta (a:i64, b:i64) -> i64{
        a-b
    }
    // ** este es el mas usado

```

el return es usado mayormente en casos donde es estrictamente necesario, pues en rust por convencion se omite el usa del return de no ser necesario.


*****
[Regresar](./Readme.md)