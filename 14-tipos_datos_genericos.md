
# Tipo de datos genericos

sirven para crear definiciones de funciones o con estructuras que pueden ser usadas en diferentes tipos de datos.


```rust
fn mayor_valor_de_una_lista<T>(list: &[T]-> T)
{
    let mut may = list[0];
    for &item in list
    {
        // el '>' no esta definido para todos los tipos de datos, por tanto esta funcion genera un error.
        if item > may{
            may = item
        }
    }
    return may
}

fn main () 
{
    let lista_numeros = vec![20,45,100,5,33,103,777,3,2,12];
    let lista_caracteres = vec!['a','c','z','y','x'];

    let may_num = mayor_valor_de_una_lista(&lista_numeros);
    println!("el mayor numero es: {}",may_num);
    let may_char = mayor_valor_de_una_lista(&lista_caracteres);
    println!("el mayor caracter es: {}",may_char);
}


```
en el ejemplo anterior nos hubiese sido de gran utilidad el uso de genericos para el control no repetir codigo, sin embargo esa solucion no es viable en este caso de rust. ahora vamos a intentar con una definicon de estructura

```rust

struct Punto<T>
{
    x:T,
    y:T,
}

struct Punto2<T,J>
{
    x: T,
    y: J
}

fn main()
{
    let entero = Punto{x:5, y:10};
    let decimal = Punto{x: 20.0, y:1,33};
    let fallara = Punto{x: 20.44, y:5};// falla por la diferencia de tipos.. ya que T es un solo tipo
    let no_fallara = Punto2{x: 20.44, y:5}; // sin embargo esto puede definir un mismo tipo para ambos

}

```

La definicion `Option<T>` uso tipos genericos al igual que `Result<T,E>` y donde sabemos ya como usarla segun sus ejemplos anteriormente visto.

Un ejemplo de como se usarian genericos en funciones es la siguiente

```rust
struct Punto<T>
{
    x:T,
    y:T,
}

impl<T> Punto<T>
{
    fn x(&self) -> &T
    {
        &self.x    
    }
}
// tambien puedo definirlo para ciertos tipos.. por ejemplo
impl Punto<f32>
{
    fn distancia_desde_origen(&self) -> f32
    {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main(){
    let punto = Punto{x:5, y:10};
    println!("la ordenada X = {}", punto.x());
    let punto_f32 = Punto<f32>{x:5.5,y:10.3};
    println!("la distancia desde el origen es= {}", punto.distancia_desde_origen());
}
```

Tambien se pueden generar combinaciones de datos entre genericos como en el ejemplo siguiente:

```rust

struct Punto<X1, Y1>
{
    x:T,
    y:T,
}

impl<X1, Y1> Punto<X2, Y2>{
    fn mezz<X2, Y2>(self, otro: Punto<X2,Y2>)-> Punto<X1, Y2>
    {
        Punto
        {
            x: self.x,
            y: otro.y,
        }
    }
}

fn main()
{
    let p1 = Punto{x: 103, y: "hola"};
    let p2 = Punto{x: "peso", y: 5};
    
    let p3 = p1.mezz(p2);
    println!("{} {}", p3.x, p3.y);

}

```
Del modo anterior podemos solventar todos los problemal generados en el ejemple inicial

## Rendimiento de tipos genericos

rust utiliza monomorfizacion a la hora de compilacion, por tanto esto reduce los costos de rendimiento. la **monomorfizacion** es el proceso de transformar el codigo escrito en genericos en codigo especifico. asi que el uso de genericos no va a limitar ni va a consumir mas equipo del ya necesario para trabajar con cualquier tipo complejo.


*****
[Regresar](./Readme.md)