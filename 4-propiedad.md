
# Propiedad
Esta es la caracteristica fundamental del lenguaje rust. Pero para poder abordar este concepto se debe de tener en cuenta lo siguiente:

#### Administracion de memoria

Para la administracion de memoria se debe de verificar los enfoques que toman diferentes lenguajes...

* **el programador** asigna y libera memoria de manera explicita(C, C++)
* **recolectores de basura**, que buscan que no se usa y lo tratan de limpiar. (Java, Python, JavaScript)
* la memora de Administra a traves de un **sistema de propiedad** con un conjunto de reglas que el conpilador verifira al momento de compilar(rust)

#### Stack vs Heap
 
* **Stack** es el concepto donde los datos se eliminan en el orden opuesto al que se obtienen; esto se le conoce como LIFO ( Last in, First Out).

* **Heap** en este espacio lo que es hace es que el administrador de memoria busca un espacio pertinente donde se pueda almacenar todo el espacio disponible y retorna en si es un puntero con la direccion de donde se ubica ese dato.

## Reglas de propiedad

1. Cada valor en rust tiene una variable que se llama propietario.
2. solo puede haber un propietario a la ves.
3. cuando el propietario se sale del ambito, el valor se eliminara.

```rust
// este ejemplo ya no es valida para las versiones actuales de rust
fn main()
{
    let cadena1 = string::from("cadena 1");
    let cadena2 = cadena1;
    
    println!("{}", cadena2); //cadena 1
    println!("{}", cadena1); // direccion de cadena 1
}

```

### Ambito de una variable

el ambito o scope de una variable es el ciclo mediante el cual este esta vivo; tipicamente es el espacio entre {} de la funcion o espacio en donde esta se ejecuta.

```rust

fn main() 
{
    {
        let cadena = string::from("hola Amigos")
        // aca si todo va genial
        println!("{}", cadena)
    }
    
    // esto genera un error de ambito
    println!("{}", cadena)
}

```

--------------

## Referencia y Prestamo

**Referencia:**

```rust
fn main() 
{
    let s = String::from("hi evenyone");
    let longitud = calculo_longitud(&s);
    println!("la longitud es: {}", longitud);
}

fn calculo_longitud(cadena: &String) -> usize
{
    let aux:usize = candeno.len();
    aux
}


```
por observacion es par algo similar al paso de parametros por referencia.. pues en este caso el paso de parametros usando el simbolo '&' hace que nuestra funcion acepte los datos de la cadeno original aun estando en un scope diferente

**Prestamo:**

    esto se basa en el principio de que si una persona tiene algo y nosotros lo necesitamos, pues se lo pedimos prestado cuando lo termine de usar.

cuando las funciones tienen referencias como parametros en lugar de los valores reales, ne necesitaremos devolver los valores para devolver la propiedad, porque realmente nunca tuvimos la propiedad...

y si necesitaramos modificar algo que tenemos prestado??

```rust
fn main() 
{
    let s = String::from("hi ");
    modificar(&s);
    println!("{}", s);
}
// esto genera un error.. por tanto no se puede modificar
fn modificar(cadena: &String) 
{
    cadena.push_str("Hector!");
}

```
Para solucionar esto se hace con datos y referencias mutables...

```rust
fn main() 
{
    let mut s = String::from("hi ");
    modificar(&mut s);
    println!("{}", s);
}

fn modificar(cadena: &String)
{
    cadena.push_str("Hector!");
}
```
    Para los **referencias mutables** se adiene una restriccion; y es que solo puede haber una refencia mutable a un dato en un ambito(scope) determinado.



```rust
fn main() 
{
    let mut s = String::from("hi ");
    lot cad1 = &mut s;
    lot cad2 = &mut s;
    // da un grave error ya que existe mas de una referencia
    println!("{}, {}", cad1, cad2);
}

```
    Por otro lado no pueden existir una referencio mutable y otra inmutable refiriendose al mismo valor

```rust
fn main() 
{
    let mut s = String::from("hi ");
    lot cad1 = &s;
    lot cad2 = &mut s;
    // aca se genera un eror por la no coexistencia de referencias..
    println!("{}, {}", cad1, cad2);
}

```
    En **resumen** por cada elemento en un scope solo puede existir una referencia (mut || ~mut) para cada valor. Pero con un cambio se puede generar un pequeno engano para esto.

* asi es algo bastante nuevo y con muchos problemas; pero ayuda a evitar **las carreras de datos** en tiempo de ejecucion.
    1.  dos o mas punteros que accedan a los mismos datos
    2. que un puntero este escribiendo en los datos y quiero otro diferente modificar/obtener los datos
    3. no se utilice un mecanismo de sincronizacion para el acceso a datos
* de otro modo tambien con esto se evita el problema de **punteros colgantes**; ocea que se usen espacios de memoria que compartan distintos programas y por tanto se reducen al maximo los errores en tiempo de ejecucion.
 
 ```rust
 fn main()
 {
    // 0! con esto se genera un error
    let ref_nothing = colgante();
    // 4! y por tanto el programa no compila por la generacion de un puntero colgante.
 }
 
 fn colgante -> &String
 {
    // 1! aqui se almaceno y genera un espocio en memoria
    let s = String::from("muere con el scope");
    // 2! devolvemos la identificacion del espacio
    &s
    //3! al terminar este scope pues este espacio se borra automaticamente
    //5! si en ves de '&s' retornamos s como tal el error desaparece..
    
 }
 
 ```
 

*****
[Regresar](./Readme.md)