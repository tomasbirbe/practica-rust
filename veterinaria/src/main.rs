use std::io::stdin;

fn main() {
    enum TipoAnimal {
        Perro,
        Gato,
    }
    struct Mascota {
        tipo: TipoAnimal,
        nombre: String,
        raza: String,
        edad: String,
    }

    struct Veterinaria {
        nombre: String,
        mascotas: Vec<Mascota>,
    }

    let mut opcion_elegida: String = String::from("");

    println!("Selecciona la opcion que corresponda");
    println!("1- Agregar veterinaria");
    println!("2- Seleccionar veterinaria");
    let inputReader = stdin();
    inputReader.read_line(&mut opcion_elegida);
    println!("{}", opcion_elegida);
}
