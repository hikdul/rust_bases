# Enumeraciones

permite definir el nombre o tipo de un dato. Para definir de usa la palabra reservaba 'enum'.

```rust
enum IPDirection{
    V4,
    V6,
}

struct IpAddress{
    tipo: IPDirection,
    address: String,
}

fn main(){

    let loopback_v4 = IpAddress{
        tipo: IPDirection::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback_v6 = IpAddress{
        tipo: IPDirection::V6,
        address: String::from("127.0.0.1::1"),
    };
}

```

si deseamos omitir la estructura podemos simplemento definirlo del siguiente modo

```rust

enum IPDirection{
    V4(String),
    V6(String),
}

fn main()
{

    let loopback_v4 = IPDirection::V4(String::from("127.0.0.1")); 
    let loopback_v6 = IPDirection::V6(String::from("127.0.0.1::1")); 

}

```

los enum son bastantes flexibles, pues pueden incluso representar un grupo de acciones o diferentes elementos.

```rust

enum Msg{
    Quitar,
    Mover {x:i32, y:i32},
    Escribir(String),
    CambiarColor(i32,i32,i32),
};

// tambien se puede generar funciones
impl Mensaje{
    fn llamar(&self)
    {
        //...
    }
}

fn main(){

}

```

## Ausente o Presente

Rust no manejo valores nulos pero en el caso de las enumeraciones si puede manejar un tipo en donde indica si un valor esta presente o ausente.

```rust

enum Option<T>{
    None,
    some(T),
}

```
la enumeracion anterior ya se encuentra dentro de la biblioteca estandar; por tanto mostrare un ejemplo de su uso

```rust

fn main()
{
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // esto generaria un error en tiempo de ejecucion ya que es pocible que 'y' tenga el valor 'None'.
    let sum = x + y;
    
}
```

*****
[Regresar](./Readme.md)