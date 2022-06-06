use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs; //file system

const FILENAME: &str = "history.csv"; // guardar path de history
const FIRST_TAG: &str = "INICIO";

// crear una estructura de datos
#[derive(Debug)]
struct DatoHistoria {
    tipo_dato: String,
    tag: String,
    texto: String,
    vida: i32,
    opciones: Vec<DatoHistoria>,
}

impl DatoHistoria {
    fn new(row: StringRecord) -> DatoHistoria {
        let vida = row.get(3).unwrap().trim();
        let vida = vida.parse().unwrap_or(0);

        return DatoHistoria {
            tipo_dato: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            texto: row.get(2).unwrap().trim().to_string(),
            vida: vida,
            opciones: vec![],
        };
    }
}
fn main() {
    let mut vida = 100;
    let mut tag_actual = FIRST_TAG.to_string();
    let mut last_record: String = "".to_string();

    let mut dato_historia: HashMap<String, DatoHistoria> = HashMap::new();

    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let dato = DatoHistoria::new(result);

        if dato.tipo_dato == "SITUACION" {
            let record_tag = dato.tag.clone();
            dato_historia.insert(record_tag.clone(), dato);
            last_record = record_tag;
        } else if dato.tipo_dato == "OPCION" {
            if let Some(data) = dato_historia.get_mut(&last_record) {
                (*data).opciones.push(dato);
            }
        }
    }

    //game loop
    loop {
        println!("Tienes {} de vida:", vida);
        if let Some(data) = dato_historia.get(&tag_actual) {
            println!("{}", data.texto);

            for (indice, option) in data.opciones.iter().enumerate() {
                println!("[{}]{}", indice, option.texto);
            }

            let mut seleccion = String::new();

            std::io::stdin().read_line(&mut seleccion).unwrap();
            let seleccion = seleccion.trim().parse().unwrap_or(99);

            if let Some(opcion_elejida) = &data.opciones.get(seleccion) {
                tag_actual = opcion_elejida.tag.clone();
            } else {
                println!("Opcion invalida");
            }

            vida += data.vida;
            println!("");

            break;
        } else {
            break;
        }

        // si la vida es igual o menor a cero
        if vida <= 0 {
           println!("Haz perdido");
        break;
        }
    }

    println!("{:?}", dato_historia["DERECHA"])
}
