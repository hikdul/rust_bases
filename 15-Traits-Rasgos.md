# Traits/Rasgos

Los traits indican al compilador una funcionalidad que tendra un tipo en particular y que se puede compartir con otros tipos. Los traits se usan para definir el comportamiento compartido de una manera abstacta, incluso definiendo estos traits como un tipo generico valido para tipos con cierto tipo de comportamiento.

Los mas similar a un trait en otros lenguajes es el uno de `interfaces` aunque si lleva con sigo algunas diferencias.

## definicion de comportamiento
el comportamiento de un tipo de dato se define como los metodos que podemos invocar en eses tipo. diferentes tipo de datos comparten el mismo comportamiento si podemos llamar a los mismos metodos en todos eses tipos.

por tanto las definiciones de traits nos permiten que diferentes tipos de datso compartan el mismo comportamiento.


```rust
// un trait
pub trait Resumen{
    fn resumir(&self) -> String;
}
// primer tipo de dato
pub Struct Noticia{
    pub titular: String,
    pub localidad: String,
    pub autor: String,
    pub contenido: String,
}
// implementacion del trait en Noticia
impl Resumen for Noticia{
    fn resumir(&self) -> String{
        format!("{}, for {} ({})",self.titular, self.autor, self.localidad)
    }
}
// segundo tipo de dato
pub Struct Tweet{
    pub usuario: String,
    pub: contenido: String,
    pub respuesta: bool,
    pub retweet: bool,
}
// implementacion de trait en tweet
impl Resumen for Tweet{
    fn resumir(&self) -> String{
        format!("{}: {} ",self.usuario, self.contenido )
    }
}
// ahora probamos su implementacion
fn main(){
    let tweet = Tweet{
        pub usuario: String::from("Hector"),
        pub: contenido: String::from("jugando con rust"),
        pub respuesta: false,
        pub retweet: false,
    };
    
    println!("tweet: {}",tweet.resumir());
    
    let noticio = Noticia{

        pub titular: String::from("rust una alternativa para ser mejor programador"),
        pub localidad: String::from("mi casa"),
        pub autor: String::from("Hector Contreras"),
        pub contenido: String::from("aca va el contenido de la noticia" ),
    }

    println!("noticia: {}",noticia.resumir());
}


```

tambien se puede definir una funcion, o el comportamiento de esta para asi tener un comportamietdo definido por defecto

```rust
pub trait Resumen{
    fn resumir(&self) -> String{
        String::from("esto es lo que hace la funcion por defecto...")
    }
}


```
igual se puede sobreescribir el metodo si por casualidad un elemento tiene o se espera un comportamiento distinto para ejecutar esta funcion

si algun metodo no tiene comportamiento por defecto y se implementa el trait en el tipo en cuestion, se debe de definir el comportamiento especifico de esta funcion.

Para utilizarlos como tipo de funcion tambien viene bien como opcion, acontinuacion un ejemplo de esto

```rust
// de este modo indico que item es cualquier valor que implemente resumen
pub fn notificar(item: &impl Resumen)
{
    println!("Ultima hora", item.resumir());
}

fn ()
{
    notificar(&tweet);
}
```

otro modo de hacer esto es:


```rust
pub fn notificar<T:Resumen>(item: &T)
{
    println!("Ultima hora", item.resumir());
}

fn ()
{
    notificar(&tweet);
}
```


*****
[Regresar](./Readme.md)