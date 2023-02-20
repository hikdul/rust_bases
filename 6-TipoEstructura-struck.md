# Struck
 
tipo de dato personalizado que permite indicar y nombrar que elementos contiene. Similar a una clase pues se puede ver como un objeto que carece de metodos.

## definicion de una estructura

se una la palabra reservada 'struck', luego el nombre de la estructura y acontinuacion los corchetes {}

```rust
struck User
{
    name: String,
    email: String,
    age: u8,
    act: bool,
}

fn main()
{
    let mut user1 = User{
        name: String::from("hikdul"),
        email: String::from("hikdul.lio@gmail.com"),
        age: 32,
        act: true,
    }
    // el usar 'mut' me permite modificarlo
    user1.email = String::from("hikdul@condigi.dev");
    
    let mut user2 = new_user("luis", "luis@yopmail.com",33);
    // usando la descomposicion tambien podemos generar datos o completar los datos faltante en base a una estructura previamente creada.
    let mut user3 = User{
        nombre: String::from("contreras"),
        email: String::from("contreras@correo.ve"),
        ..user1
    }
}

fn new_user (name: String, email: String, age:u8) ->{
    // para la asignacion de valores; para cuando este contienen el mismo nombre se puede hacer similor a JS
    User{
        name,
        email,
        age,
        activo: true,
    }
}

```


## estructuras y tuplas

puede tener diferentes tipos de datos; con la diferencia de que en las tuplas accedemos por medio de un indice y en las estructuras accedemos a ellos por medio del nombre.

Ahora bien si las mesclamos podemos obtener las llamadas estructuras de tuplas

### estructuras de tuplas

son basicamente una mezcla de ambos.. es algo asi como nombrar los datos de una estructura y luego para los datos no tenemos nombre para los datos

```rust
// en esta ideo nos basamos en los colores RGB donde el orden seria
// Color.0 = cantidad de amarillo
// Color.1 = cantidad de azul
// Color.2 = cantidad de rojo

struct Color(i32,i32,i32);

fn main(){
    
    let negro = Color(0,0,0);
}

```
### Estructuras de tipo de unidad

estas son estructuras que carecen de tipo; Tipicamente usadas para defiris algun rasgo...

## Metodos

es similar a las funciones con la unica diferencia de que estos van a ser declarados dentro de una estructura, emumeracion u objeto de rango; y siempre el primer parametro el 'self'.

```rust
 struct Rectangulo
 {
    ancho: u32,
    alto: u32,
 }

impl Rectangulo{
    fn area(&self) -> u32
    {
        self.ancho * self.alto
    }
    
}
// pueden ir en un mismo bloque de implementacion o en uno distinto

impl Rectangulo{

    // si un rectangulo cabe dentro de otro
    fn puede_contener(&self, otro: &Rectangulo)-> bool
    {
        self.ancho > otro.ancho && sefl.alto > otro.alto
    }
}

fn main(){

    let rec1 = Rectangulo{
        ancho: 55,
        alto: 65
    };
    println!("El Area es = {}", rec1.area());
    

    let rec2 = Rectangulo{
        ancho: 45,
        alto: 85
    };

    let rec3 = Rectangulo{
        ancho: 25,
        alto: 25
    };
    
    if rec1.puede_contener(&rec2)
        println!("si contiene rectangulo 2");
    if rec1.puede_contener(&rec3)
        println!("si contiene rectangulo 3");

}

```

### funciones asociadas

son funciones que **no contienen el parametro self**; un ejemplo claro es el uso de `String::form("alguna cadena")` pues esta es en si una funcion asociada. Un uso muy comun es para el uso de contructores


```rust

 struct Rectangulo
 {
    ancho: u32,
    alto: u32,
 }
 
 impl Rectangulo
 {
    fn cuadrado(lado:u32) -> Rectangulo
    {
        Rectangulo
        {
            ancho: lado,
            alto: lado
        }
    }   
 }
 
 fn main(){
    let cuadrado = Rectangulo::cuadrado(5);
 }
 
 ```

*****
[Regresar](./Readme.md)