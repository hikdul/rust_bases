# Vectores

son estructuras de memoria que almacenan colecciones de datos en una unica estructura, en memoria colocan todos los valores en posiciones consecutivas. Y los datos que se almacenan son siempre de un mismo tipo.

```rust
// asi se declara un vector
let mut v: vec<i32> = Vec::new();
// add algun elemento en el vector
v.push(5);
v.push(500);
v.push(50);

println!("La longitud del vertor es: {}", v.len());

// extrae el ultimo elemento del vector
v.pop();

// * Otra forma de declararlos seria la siguiente:
let asd = vec![1,2,3,4,5,6,7,8,9,0];// de este modo rust toma gracias a sus elementos el tipo de dato y nos genera un vector con los elementos ya definidos

// Para obtener un dato especifico se hace con el pocicionamiento normal partiendo siempre de 0
let tercero: &i32 = &asd[2];
println!("el tercer elemento es: {}", tercero)

// otro es usando el operador get

match asd.get(3){
    Some(tercero) => println!("SI coinciden"),
    None => println!("NO coinciden"),
}


```

## manejo de errores

1. no se puede acceder a una posicion que no tenga datos o no este asignado a un vector; pero si obtenemos el valor de una pocicion no asignada por ejemplo `let last = asd[999999999];` nos da un error de compilacion; pero si lo obtenemos con get ` let last = asd.get(999999999)` solo nos entrega un warning. Esto se debe a que con el get nos retorna el valor none mientra que con el indice nos da literalmente un error.
2. no se puede asignar otro valor a un vector mientras este mantenga una referencia a el mismo. Esto se debe a que si por casualidad la nueva posicion del vector esta ocupada y por tanto se mueve el elemento a una nueva ubicacion de memoria entonces la referencia anterior causaria un grave error

```rust
fn main(){
    let mut v = vec![1,2,3,4,5];
    let primero = &v[0];
    v.push(6);// aqui generaria el error 
    println!("el primer valor es: {}",primero);
}

```

entre otros...

## Iterator
par iterar un vector es similar a otros lenguajes
```rust

fn main(){
    let mut v = vec![1,2,3,4,5];
    // una iteracion normal
    for i in &v{
        prinln!("{}",i)
    }

    // si lo quisiera modificar
    for i in &mut v{
        *i += 10;
    }
}
    
```

## tipos abstractos

una forma para agregar tipos abstractos o diferentes valores en un array puese ser usando enums

```rust

enum CeldaHojaCalculo{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main(){
    
    let fila = vec![
        CeldaHojaCalculo::Int(9),
        CeldaHojaCalculo::Text(String::from("texto de ejemplo")),
        CeldaHojaCalculo::Float(9.81),
        CeldaHojaCalculo::Int(99),
    ]
}

```


*****
[Regresar](./Readme.md)