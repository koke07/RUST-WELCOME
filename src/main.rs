use regex::Regex;
/*fn main() {
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
}*/
/*fn main(){
    println!("Happy coding, ingresa por favor tu nombre: ");
    //let mut nombre: String = String::from("Jorge"); 
    let mut nombre: String = String::new(); 
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();
    //libreria estandar , input and output,std in para informacion, leer datos desde la consola,se van a guardar en nombrar , es para manejar errores
    print!("Hola, bienvenid@ {}", nombre);
    edad()
} */

/*fn edad(){
    println!("Happy coding, ingresa por favor tu edad: ");
    //obtener la edad de la consola 
    let mut edad: String = String::new(); 
    std::io::stdin().read_line(&mut edad).unwrap();
    // convertirla a numero

    let edad_int: u8 = edad.trim().parse().unwrap();

    if edad_int <= 18{
        println!("Eres menor de edad, no puedes ingresar a la Disco");
    }
    else{
       print!("Hola bienvenido a la Disco, edad {} años", edad_int); 
    }


    
}*/



fn main(){
   //form_foraneo();
   //edad();
   pildoras();
}

/*fn form_foraneo(){
    println!("Happy coding, ingresa por favor tu nombre: ");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();
    println!("Happy coding, ingresa por favor tu pais de origen: ");
    let mut pais: String = String::new();
    std::io::stdin().read_line(&mut pais).unwrap();
    pais = pais.trim().to_string();
    print!("Hola, bienvenido {} tu pais de origen es {} ", nombre,pais);
}*/

fn pildoras(){
    println!("Hola bienvenido, que pastillas desea tomar, azul o roja?");
    let mut pastillas: String = String::new();
    std::io::stdin().read_line(&mut pastillas).unwrap();
    pastillas = pastillas.trim().to_lowercase();

    if pastillas == "azul"{
        println!("Felicidades, usted elijio pastillas azules")
    }
    else if pastillas == "roja"{
        println!("Felicidades, usted elijio pastillas rojas")
    }
    else{
        println!("Lo sentimos, no tenemos esa pastilla")
    }
    
}