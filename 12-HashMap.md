# HashMap

el tipo HashMap<K,V> almacena una asignacion de key(k), value. Este es similar a los *dictyonary* en C#. Pero esto es para situar mediante un valor hash que almacena datos en un vector.

```rust


fn simple()
{

    use std::collections::HashMap;
    
    let dic = HashMap::new();
    dic.insert(String::from("La Lakers"), 222);
    dic.insert(String::from("Warriors"), 221);
}


fn complejo()
{
    // asi agregas una libreria
    use std::collections::HashMap;
    
    let equipos = vec![String::from("Deportivo Tachira"),String::from("Alianza Lima")];
    let puntuaciones = vec![256, 12];
    
    // esto es una forma mas iterada
    let mut dic: HashMap<_,_> = equipos.into_iter().zip(puntuaciones.into_iter()).collect();
    
}


```

**NOTA:** si un elemento tiene la habilidad de .copy() se usa sin restriccion alguna, pero en el caso de aquellos elementos que no contienen esta propiedad como es el caso de los String el objeto se transfiere completamente al hashmap; por tanto si usamos us string para dar valor a algun elemento dentro del hashMap, pues este luego no puede ser usado ya que esta completamente transferido al valor del hashMap.

### Acceder a un valor
por otro lado para acceder a los datos dentro de un hashMap se puede hacer mediante la funcion get o iterando todos los elementos

```rust
fn main()
{
    use std::collections::HashMap;
    
    let mut puntos = HashMap::new();
    
    puntos.insert(String::from("Equipo uno"),121);
    puntos.insert(String::from("Equipo dos"),122);
    puntos.insert(String::from("Equipo tres"),123);
    puntos.insert(String::from("Equipo cuatro"),124);
    
    // usando get
    let nombre_equipo = String::from("Equipo cuatro");
    let puntuacion = puntos.get(&nombre_equipo);
    
    //iterando
    for (key, value) in &puntos{
        println!("{}: {} \n",key, value);
    }
}   

```
### Editar, sobre escribir un valor

* por medio de la sobre escritura de su clave

```rust

    use std::collections::HashMap;
    
    let mut puntos = HashMap::new();
    
    puntos.insert(String::from("Equipo uno"),121);
    puntos.insert(String::from("Equipo dos"),122);
    puntos.insert(String::from("Equipo tres"),123);
    puntos.insert(String::from("Equipo cuatro"),124);
    // -- sobre escribo equipo uno y tres
    puntos.insert(String::from("Equipo uno"),551);
    puntos.insert(String::from("Equipo tres"),553);

```
* verificar si una clave tiene X valor y en caso de no tenerlo lo sobre escribimos

```rust

    use std::collections::HashMap;
    
    let mut puntos = HashMap::new();
    
    puntos.insert(String::from("Equipo uno"),121);
    puntos.insert(String::from("Equipo dos"),122);
    puntos.insert(String::from("Equipo tres"),123);
    puntos.insert(String::from("Equipo cuatro"),124);
    // hago la validacion y si no existiera le agrego el valor 0
    puntos.entry(String::from("Equipo cero")).or_insert(0); //==> esto sucede
    puntos.entry(String::from("Equipo uno")).or_insert(1); //==> esto no sucede por que ya tenia un valor
    
    // asi logro que me muestre todos los valores del hashMap con el uso de {:?}
    println!("{:?}",puntos);

```

## un ejemplo de pontenciales de uso

como siempre se dice la limitacion depende de quien lo piense, pero los hashMaps no estan limitados solo para tener una clave y valor.. tambien podriamos tener ejercicios practicos.. en el siguiente caso vamos a contar cuantas veces se repite cada palabra en una cadena de texto usando exclusivamente los hashMap

```rust

    use std::collections::HashMap;
    
    let texto = "hola mundo maravilloso mundo mundo contaminado nuestro mundo nos dice hola en las mañana pero el grocero no dice nada al anochecer";
    
    let mut map = HashMap::new();
    for palabra in texto.split_whitespace(){
        let cont = map.entry(palabra).or_insert(0);
        *cont += 1; w
    }
    
    println!("{:?}",map);

```

*****
[Regresar](./Readme.md)