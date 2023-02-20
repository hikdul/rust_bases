# Operador De Control De Flujo match

es un operador que nos permite compara un valor con una serie de patrones y luego si coincide ejecutar algun codigo.

los patrones pueden ser definidos por valores literales, nombres de variables o comodines, etc...


```rust

#[derive(Debug)]

enum Mount{
    Enero, 
    Febrero,
    Marzo,
    //....
   
}

enum Time{
    Segundo,
    Minuto, 
    Hora,
    Dia(Mount)
}

fn valor_en_segundo(tiempo: Time) ->u32{
    match tiempo{
        Time::Segundo => 1,
        Time::Minuto => 60,
        Time::Hora => 3600,
        Time::Dia(Mount) =>{
            println!("este dia corresponde al mes {:?}",Mount);
            24 * 3600
        },
    }
}

fn main()
{
    let dia_january = Time::Dia(Mes::Enero);
    let tiempo_sg = valor_en_segundo(dia_january);
}

```
un ejemplo seria usar el enum Option<T> para ilustrar un poco el uso de **match**

```rust

fn incrementar_uno(x: Option<i32>) -> Option<i32>
{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main()
{
    let cinco = Some(5);
    let seis = incrementar_uno(cinco);
    let nada = incrementar_uno(None);
}

```
Uno dato importante es que al utilizar el operador **match** sobre un emum, se debe de cubrir todos los casos en caso de no hacerlo el compilador generara un error de "patron no cuvierto completamente"

un ejemplo de **match** mucho mas explicito y que indica el potencial de este operador es la del uso en diferentes rangos

``` rust

fn una_jugada_sin_tirar()
{
    // ...
}

fn vuelve_a_tirar(){
    // ...
}

fn avanza(num_celdas:u8){
    // ...
}

fn main(){

    let dado = 5;
    
    match dado{
        1 => una_jugada_sin_tirar(),
        6 =>vuelve_a_tirar(), 
        other => avanza(other),
        // en el caso de que la fn 'avanza' no recibiera parametros se puede sustituir otrer por _(underScore)
    };
}


```

*****
[Regresar](./Readme.md)