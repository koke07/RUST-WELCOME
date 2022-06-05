fn main() {
    let nombre: &str ="Jorge";
    let edad:i8 = 22;
    let temp_max:i16 =19;
    let temp_min:i16 = 7;
    saludo(nombre,edad);
    temperatura(temp_max, temp_min)
}

fn saludo(nombre: &str, edad:i8){
     println!("Hola {}, me encanta tu código!",nombre);
     println!("{}, tu edad es {}", nombre,edad) 
}

fn temperatura(temp_max:i16, temp_min:i16){
    println!("La temperatura máxima en Bogotá es {}° y la mínima es {}°",temp_max,temp_min);
}