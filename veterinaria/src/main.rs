use std::io::stdin;

#[derive(Debug)]
enum TipoAnimal {
    Perro,
    Gato,
}
#[derive(Debug)]
struct Mascota {
    tipo: TipoAnimal,
    nombre: String,
    raza: String,
    edad: String,
}
#[derive(Debug)]
struct Veterinaria {
    nombre: String,
    mascotas: Vec<Mascota>,
}

impl Veterinaria {
    fn new(nombre: String, mascotas: Vec<Mascota>) -> Veterinaria {
        return Veterinaria {
            nombre: String::from(nombre),
            mascotas,
        };
    }
}

fn agregarVeterinaria(veterinarias: &mut Vec<Veterinaria>) {
    println!("Creating new veterinaria...");
    veterinarias.push(Veterinaria::new(
        String::from("Veterinaria la pichicata"),
        vec![],
    ));
    println!("New veterinaria created");
}

fn menu(veterinarias: &mut Vec<Veterinaria>) {
    let mut opcion_elegida: String = String::from("");

    println!("Selecciona la opcion que corresponda");
    println!("1- Agregar veterinaria");
    println!("2- Seleccionar veterinaria");
    let input_reader = stdin();
    input_reader.read_line(&mut opcion_elegida).unwrap_or(0);
    if opcion_elegida.trim() == "1" {
        agregarVeterinaria(veterinarias);
    }
}

fn main() {
    let mut veterinarias: Vec<Veterinaria> = vec![];

    menu(&mut veterinarias);
    println!("{:#?}", veterinarias);
}
