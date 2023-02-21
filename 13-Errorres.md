
# Errores

Para el manejo de error Rust presenta la macro `panic!`... 

una ves llamado el programa ara las siguiente acciones:
1. imprime el mensaje con el fallo
2. se desenreda y limpia la pila
3. se sierra el programa

En caso de que se necesite que el binario pese lo menos posible se puede cambiar la opcion de desenredar por **abordar** en el caso de panico. Asi evitamos el trabajo que conlleva generar esta limpieza, sin embargo este trabajo quedaria en manos del sistema operativo. Para activa esta opcion en el archivo `Cargo.toml` se agrega las siguientes lineas:

```
    [profile.release]
    panic = abort

```

Para llamar a panic es tan simple como ejecutar una funcion, como se observa en el siguiente ejemplo:

```rust

fn main()
{
    panic!("Error Provocado")
}
```
ovbiamente Rust llama a panic en los casos que estan anteriormetne programados:

```rust
fn main()
{
    let v= vec![1,2,3];
    let aux = v[100]; // esto llama a panic
}

```

# Recuperar errores

Para recuperar errores tenemos el enumerador `Result`

```rust

enum Result<T,E>
{
    Ok(T),// T => tipo generico; Ok con que no hay error.
    Err(E), // E => de error; Para el manejo del error (creo)
}

```
ahora tenemos un ejemplo donde vamos a leer un fichero que no existe


```rust
 
use std::fs::File;

fn main()
{
    let f = File:open("hola.txt");
    let f = match f {
        ok(fichero) => fichero,
        Err(error) => panic!("Error el archivo no existe, Error = {:?}", Error);
    }
}

```
Para los errores tambien se pueden manejar basandonos en su tipo


```rust
 
 // en este caso vamos a intentar acceder a in fichero; si este no existe pues lo creamos; en caso de error lo retornamos
 
use std::fs::File;
// con esto manejamos el tipo de errores
use std::io::ErrorKind;

fn main()
{
    let f = File:open("hola.txt");
    let f = match f {
        ok(fichero) => fichero,
        Err(error) => match error.Kind(){
            // con errorKind manejamos el tipo de error basados en la informacion que queremon controlar
            // aca si el error es por que el archivo no existe pues lo creamos
            ErrorKind::NotFound => match File::create("hola.txt"){
                OK(fcreated) => fcreated,
                Err(e) => panic("Error desconocido al crear el archivo: {:?}", e),
            },
            other_error =>{
                panic!("Error el archivo no existe, Error = {:?}", other_error);
            }
        } 
    }
}

```

Existen metodos para que el codigo no quede tan dificil de leer; por ejemplo el metodo `unwrap()`. este metodo lo que hace es generar es devolver el valor de un elemento si no generar ningun error, en caso de error entra directamente a panico:

```rust

use std::fs::File;
use std::io::ErrorKind;

fn main()
{
    let f = File:open("hola.txt").unwrap();
}

```

tambien hay un metodo `.expect("...");`, donde este metodo nosotros colocamos el error que se desea mostrar:

```rust

use std::fs::File;
use std::io::ErrorKind;

fn main()
{
    let f = File:open("hola.txt").expect("no existe el archivo hola.txt");
}

```
### Propagacion de errores

```rust

use std::fs::File;
use std::io::{self, Read};

fn reas_user_from_file() -> Result<String, io::Error>
{
    let f = File::open("hola.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s String:new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}

fn main()
{
}

```
refactorizo ...

```rust

use std::fs::File;
use std::io::{self, Read};

fn reas_user_from_file() -> Result<String, io::Error>
{
    let f = File::open("hola.txt");
    let mut s String:new();
    match f.read_to_string(&mut s)?;
    Ok(s)
}

fn main()
{
}

```
volvemas a reducir el codigo


```rust

use std::fs;
use std::io;

fn reas_user_from_file() -> Result<String, io::Error>
{
    fs::read_to_string("hola.txt");
}

fn main()
{
}

```

*****
[Regresar](./Readme.md)