

 ## Variables en rust   
para declarar una variable se la palabra reservada 'let'. las variables son inmutables, ocea que no cambian su valOr

    ```rust
    let var = 3;
    println!("el valor de 'var' es = {}", var);
    ```
para que una variable sea mutable se debe de acompanar de la palabra 'mut'

```rust
    let mut mutable = 3.2;
    println!("el valor de 'mutable' es = {}", mutable);
    mutable = 6.33;
    println!("el valor de 'mutable' es = {}", mutable);
 ```
    
para definir una constante se hace el con la palabra 'const'

 ```rust 
    let mutable = 3.2;
    const PI:f64 = 3.1416;
    println!("el valor de PI es = {}", PI);
    
    println!("PI X mutable = {}", PI*mutable);
``` 
para reasignar valores, se reescribe la variable

    ```rust
    let var: char = 'c';
    println!("el valor de 'var' es = {}", var);
    ```
    
# Tipos de datos en rust 

Para rust la tipologia es un poco especial pero realmente se puede llamar intuitiva

## Enteros
los numeros enteros se pueden definir ya sea por su valor con signo o sin signo(solo positivos).

para saber el rango de valores se puede apreciar en base a la siguiente formulas
```
sin signo = 0 ... 2^(n) - 1;
con signo = 2^(n-1) ... 2^(n-1) - 1;
```
y siguiendo la siguiente tabla mediante la cual asignamos la variable

<table>
<thead>
<tr>
    <th>Longitud</th>
    <th>Con Signo</th>
    <th>Sin Signo</th>
</tr>
</thead>
<tbody>
<tr>
    <td>
        8 bit
    </td>
    <td>
        i8
    </td>
    <td>
        u8
    </td>
</tr>
<tr>
    <td>
        16 bit
    </td>
    <td>
        i16
    </td>
    <td>
        u16
    </td>
</tr>
<tr>
    <td>
        32 bit
    </td>
    <td>
        i32
    </td>
    <td>
        u32
    </td>
</tr>
<tr>
    <td>
        64 bit
    </td>
    <td>
        i64
    </td>
    <td>
        u64
    </td>
</tr>

<tr>
    <td>
        128 bit
    </td>
    <td>
        i128
    </td>
    <td>
        u128
    </td>
</tr>

<tr>
    <td>
        ---
    </td>
    <td>
        isize
    </td>
    <td>
        usize
    </td>
</tr>
</tbody>
</table>

```
siendo por ejemplo en el caso de los 8 
sin signo = 0 ... 2^(n) - 1 ==> [0 ... 256]
con signo = 2^(n-1) ... 2^(n-1) - 1; === [-128 ... 127]
```

y asi sucesivamente dependiendo del tipo de entero que sea declarado
 
los tipos **isize** o **usize** ya los valores se asignan basados en la plataforma en la que se genere la aplicacion. esto ya es algo particular de este lenguaje
 
## Tipos en coma flotante

para esto tipos la definiciones son 
<table>
<thead>
<tr>
<th>Declaracion</th>
<th>bits</th>
</tr>
</thead>
<tbody>
<tr>
<td>
f32
</td>
<td>
32 bits
</td>
<td>
f64
</td>
<td>
64 bits
</td>
</tr>
</tbody>
</table>

son como float(f32) y double(f64) en C++

## Operaciones basicas matematicas

estas son los mismas que en cualquier lenguaje:
suma: +
resta: -
multiplicacion: *
divicion: /
resto: %

## Booleano o Logico

solo almacena 2 valores (true/false) es almacenado como un byte y se define con la palabra reservada **bool**
``` rust
let esta:bool = false
if esta
    println!("si esta");
else
    println!("no esta")
    
```

## Tipo Caracter

se define mediante la palabra reservada **char** y se asigna su valor usando las comillas simples ''. tambien sirve para asignar letras acentuadas, caracteres de otros idiomas o emojis; esto se debe a que los caracteres se almacenan en 4 bits por tanto permite almacenar cualquier caracter de unicode.

```rush
 let caracter:char = 'c';
```

## Tipos compuestos
esto es para almacenar varios valores en un solo tipo; solo se permite almacenar tipos primitivos. Estas son las famosas tuplas y arrays(matrices).

### tuplas
Permite almacenar valores de diferentes tipos separados por comas, su longitud es constante y esta se define una ves se declara.

```rust
    let tupla:(i32, u32, char) = (500, 34, 'c');
    // para desestructurar
    let(x,y,z)= tupla
    // para asignar un dato a una variable se hace indicando su posicion dentro de la tupla
    let enteroPositivo:u32 = tupla.1
     
```
### arays/matriz

permite almacenar una coleccion de datos multiples siempre y cuando estos sean del mismo tipo; en cuanto al tamano es siempre el mismo...

```rust
    // en la asignacion [a;b] donde 'a' es el tipo y 'b' es la longitud
    let matriz:[i32;6] = [1,0,3,44,55,-1];
    // para obtener un dato
    let value:i32 = matriz[3]; // de este modo value = 44
    // otro modo de declarar una matriz es
    let matrizEqual= [3;5]// de este modo matrizEqual = [3,3,3,3,3]
```

    
*****
[Regresar](./Readme.md)
