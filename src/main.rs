use regex::Regex;
fn main() {
    print!("Hola, Bienvenido a mi calculadora!\n");
    // expresiones regulares que vamos a utilizar
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_rest = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    

    // datos del usuario que vamos a utilizar
    print!("Por favor , indica su expresion!\n");
    let mut expresion = String::new();
    std::io::stdin().read_line(&mut expresion).unwrap();
    // aplicar operaciones

    //multiplicacion
    loop {
        let caps = re_mult.captures(&expresion);
        if caps.is_none(){
            break;
        }
        let caps = caps.unwrap();


        let cap_expression= caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let mult = left_value * right_value;
        expresion = expresion.replace(cap_expression, &mult.to_string());
        
        print!("{:?}izq,{:?}der", left_value, right_value);
    }
    // suma 

    loop {
        let caps = re_add.captures(&expresion);
        if caps.is_none(){
            break;
        }
        let caps = caps.unwrap();


        let cap_expression= caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition = left_value + right_value;
        expresion = expresion.replace(cap_expression, &addition.to_string());
        
        print!("{:?}izq,{:?}der", left_value, right_value);
    }
    //resta
    loop{
        let caps = re_rest.captures(&expresion);
        if caps.is_none(){
            break;
        }
        let caps = caps.unwrap();
        let cap_expression= caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let rest = left_value - right_value;
        expresion = expresion.replace(cap_expression, &rest.to_string());
        
        print!("{:?}izq,{:?}der", left_value, right_value);
    }
    //division
    loop{
        let caps = re_rest.captures(&expresion);
        if caps.is_none(){
            break;
        }
        let caps = caps.unwrap();
        let cap_expression= caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let div = left_value / right_value;
        expresion = expresion.replace(cap_expression, &div.to_string());
    }


    // mostrar nuestro resultado
    print!("El resultado de la operacion es:{}",expresion);
}
