use std::{error::Error, io::stdin, num::ParseIntError, option};

#[derive(Debug)]
enum TipoAnimal {
    Perro,
    Gato,
}
#[derive(Debug)]
struct Pet {
    tipo: TipoAnimal,
    nombre: String,
    raza: String,
    edad: String,
}
#[derive(Debug)]
struct Vet {
    name: String,
    pets: Vec<Pet>,
}

impl Vet {
    fn new(nombre: String, pets: Vec<Pet>) -> Vet {
        return Vet {
            name: String::from(nombre),
            pets,
        };
    }
}

fn add_vet(veterinarias: &mut Vec<Vet>, vet_name: String) {
    veterinarias.push(Vet::new(vet_name, vec![]));
}

fn menu(options: &Vec<&str>) -> String {
    let mut selected_option = String::from("");
    let input_reader = stdin();
    for option in options {
        println!("{}", option);
    }
    input_reader.read_line(&mut selected_option).unwrap_or(0);
    return String::from(selected_option.trim());
}

fn select_vet(vets: &Vec<Vet>) -> Result<usize, ParseIntError> {
    let mut menu_number = 1;
    println!("Seleccionar una veterinaria");
    for vet in vets.iter() {
        println!("{} - {}", menu_number, vet.name);
        menu_number += 1;
    }
    let input_reader = stdin();
    let mut option_selected = String::from("");
    input_reader.read_line(&mut option_selected).unwrap_or(0);
    return option_selected.trim().parse::<usize>();
}

fn main() {
    let mut vets: Vec<Vet> = vec![];
    let principal_options = vec![
        "0- Salir",
        "1- Agregar veterinaria",
        "2- Seleccionar veterinaria",
    ];
    loop {
        println!("\n");
        match menu(&principal_options).as_str() {
            "1" => {
                println!("\n");
                let add_vet_options = vec!["Escribi el nombre de la veterinaria"];
                let new_vet_name = menu(&add_vet_options);
                add_vet(&mut vets, new_vet_name);
            }
            "2" => {
                println!("\n");
                let mut selected_vet: &Vet;
                // loop {
                match select_vet(&vets) {
                    Ok(number) => {
                        let a = vets.get(number - 1).unwrap();
                        match a {
                            Some() => todo!(),
                            
                        }
                    }
                    Err(_) => todo!(),
                };
                // }
            }
            "0" => {
                println!("\n");
                println!("Adios!");
                break;
            }
            _ => {
                println!("\nIngresa una opcion valida");
            }
        }
    }
}
