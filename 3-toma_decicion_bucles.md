# Expreciones if

para la ramificacion y generacion de limitanto o condiciones en if

```rust

let verdad = false

if verdad
    println!("ES Verdad")
else
    println!("ES Mentira")

```

en rust no existe el operador ternario pero si permite generar algo similar como en el siguiente codigo

```rust
   let condicion = true;
   let numero = if condicion {5} else {-5};
   println!("el valor de numero es: {}", numero);
```

# Bucles

hay tres tipos:

1. **loop:** para hacer recorridos infinitos y que se rompen con la sentencia 'break'. Aunque no es algo altamente recomendado en loop la diferencia radica en que este loop puede devolver algun valor al resto del codigo.

```rust

let mut contador = 0;

let result = loop{
    println!("Hi, Friends!! ===> {}", contador);
    contador += 1;
    if contador >= 10
    {
        break;
    }

}

println!("result of loop is = {}", result);

```

2. **while:** sirve para generar bucles que se rompan mediante una condicion

```rust
    let mm = [10,20,39,42,56,66,67,75,85,91,9];
    let mut i = 0;

    while i < 9
    {
        println!("valor = {}, index = {}", mm[i], i);
        i+=1;
    }
    
```

3. **for in:** Para recorrer o generar un loop mediante el uso de un iterator.

```rust
// de este modo es mas parecido a lo que seria el foreach en otros lenguajes
    let mut mm = [10,20,39,42,56,66,67,75,85,91,9];
    for numero in mm.iter()
    {
        println!("valor = {}", numero);
    }

```
otro modo de iterar es recorriendo con valores literales; por ejemplo aca se observa como se puede tener un listado de numeros desde el 1 hasta el 10

```rust
    for number in 1..10
    {
        println!(" {} ,", number);
    }
```
y para obtenerlo de manera contraria usariamos uno de los metodos del lenguaje dentro del sistema; en este caso el metodo .rev()... asi obtenemos los digitos del siguiente modo 10, 9 ... 1

```rust
    for number in (1..10).rev()
    {
        println!(" {}, ", number);
    }
```


*****
[Regresar](./Readme.md)
