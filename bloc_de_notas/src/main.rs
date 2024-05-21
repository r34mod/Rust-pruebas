use std::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{self, Read, Write};

const JSON_FILE: &str = "document.json";

struct Nota {
    id: usize,
    content: String,
}

fn read_doc() -> Vec<Nota> {
    let mut archivo = match File::open(JSON_FILE){
        Ok(file) => file,
        Err(_) => return Vec::new()
    };

    let mut content = String::new();
    file.read_to_string(&mut content).expect("No se pudo leer");

    let notas: Vec<Nota> = serde_json::from_str(&content).unwrap_or_else(|e| Vec::new());
    notas
}


fn write_doc(notas: &Vec<Nota>){
        let content = serde_json::to_string_pretty(notas).expect("No se pudo serializar");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(JSON_FILE)
            .expect("No se pudo abrir el fichero");

        file.write_all(content.as_bytes()).expect("No se pudo escribir en el archivo");

}


fn add_nota(notas: &mut Vec<Nota>, content: String){
    let id = if let Some(nota) = notas.last() {
        nota.id + 1
    }else {
        1
    };

    let nota = Nota { id, content };
    notas.push(nota);
    write_doc(notas);
}


fn list_notas(notas: &Vec<Nota>){
    for nota in notas {
        println!("ID: {}\nContent: {}\n", nota.id, nota.content);
    }
}

fn delete_nota(notas: &mut Vec<Nota>, id: usize){
    if let Some(pos) = notas.iter().position(|x| x.id == id){
        notas.remove(pos);
        write_doc(notas);
    }else{
        println!("Nota con ID {} no se encuentra", id);
    }
}


fn main(){
    let mut notas = read_doc();

    loop{
        println!("1. Add nota");
        println!("2. List notas");
        println!("3. Delete nota");
        println!("4. Exit");
        println!("Select an option ");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("No se pudo leer");
        let option: u32 = option.trim().parse().expect("Ingresa un numero");

        match option {
            1 => {
                println!("Ingrese el contenido de la nota:");
                let mut contenido = String::new();
                io::stdin().read_line(&mut contenido).expect("No se pudo leer la línea");
                let contenido = contenido.trim().to_string();
                agregar_nota(&mut notas, contenido);
            }
            2 => {
                listar_notas(&notas);
            }
            3 => {
                println!("Ingrese el ID de la nota a eliminar:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("No se pudo leer la línea");
                let id: usize = id.trim().parse().expect("Por favor, ingrese un número");
                eliminar_nota(&mut notas, id);
            }
            4 => {
                break;
            }
            _ => {
                println!("Opción no válida. Por favor, intente de nuevo.");
            }
        }
    }
}

