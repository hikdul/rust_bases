
fn main()
{

    primeros_pasos();
    
    println!("--------------------------------- Separator ---------------------------------");
    // la declaracion ejecuta una accion y no retorna valor alguno
    declaracion(33);
    // la exprocion evalua y retorna el valor resultante
    exprecion_ex(10);
    
    println!("--------------------------------- Separator ---------------------------------");

    example_propiedad();
    
    println!("--------------------------------- Separator ---------------------------------");
    
    example_hm();
    
    println!("--------------------------------- Separator ---------------------------------");

}

fn exprecion_ex(i:i32)
{

    let j = {
        let k = i+1;
        k+1
    };
    println!("el valor de J = {}", j);
}

fn declaracion(i:i32)
{
   println!("el valor ingresado es: {}",i) ;
}

fn primeros_pasos (){

    // ! en este archivo compondre los elementos basicos de rust
    println!("Hello, world!");
    
    // - para declarar una variable se la palabra reservada 'let'
    // ** las variables son inmutables, ocea que no cambian su valOr
    let var = 3;
    println!("el valor de 'var' es = {}", var);
    
    // - para que una variable sea mutable se debe de acompanar de la palabra 'mut'

    let mut mutable = 3.2;
    println!("el valor de 'mutable' es = {}", mutable);
    mutable = 6.33;
    println!("el valor de 'mutable' es = {}", mutable);
    
    // - para definir una constante se hace el con la palabra 'const'
    
    const PI:f64 = 3.1416;
    println!("el valor de PI es = {}", PI);
    
    println!("PI X mutable = {}", PI*mutable);
    
    // - para reasignar valores, se reescribe la variable
    let var: char = 'c';
    println!("el valor de 'var' es = {}", var);
}

fn example_propiedad()
{
    let cadena1 = String::from("cadena 1");
    let cadena2 = cadena1.clone();
    
    println!("{}", cadena2); //cadena 1
    println!("{}", cadena1); // direccion de cadena 1
}

fn example_hm()
{

    use std::collections::HashMap;
    
    let texto = "hola mundo maravilloso mundo mundo contaminado nuestro mundo nos dice hola en las mañana pero el grocero no dice nada al anochecer";
    
    let mut map = HashMap::new();
    for palabra in texto.split_whitespace(){
        let cont = map.entry(palabra).or_insert(0);
        *cont += 1; 
    }
    
    println!("{:?}",map);

}
