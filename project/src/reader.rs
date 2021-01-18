use std::{error::Error, vec};
use std::fs;
use std::process;

use regex::Regex;

use serde::Deserialize;
use serde::Serialize;

use crate::database::database;
use crate::database::insert;

#[derive(Debug, Deserialize, Serialize)]
pub struct Persona {
    pub identificacion: String,
    pub nombre: String,
    pub genero: String,
    pub estadocivil: String,
    pub fechanacimiento: String,
    pub telefono: String,
    pub direccion: String,
    pub email: String
}

pub fn read() -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir("./tmp/").unwrap(); 
    
    if let Err(err) = database() {
        println!("{}", err);
        process::exit(1);
    }

    for path in paths { 
        let dir = path?;
        let mut rdr = csv::ReaderBuilder::new()
                                        .has_headers(false)
                                        .delimiter(b';')
                                        .from_path(dir.path())
                                        .expect("Cant read field");
        for result in rdr.deserialize::<Persona>() {
            let record = result?;
            let mut validado: i32 = 1;
            let mut observacion = String::new();
            if validarcedula(record.identificacion.clone()) == false {
                let repasaporte = Regex::new(r"^[ÑA-Z0-9]+$").unwrap();
                if !repasaporte.is_match(&record.identificacion) {
                    observacion.push_str("Numero de identificacion no valido - Uso de caracteres no validos\n");
                } else {
                    if record.identificacion.chars().count() < 5 || record.identificacion.chars().count() > 13 {
                        observacion.push_str("Numero de identificacion no valido - Longuitud no valida\n");
                    } 
                }
            }

            if record.nombre.split_whitespace().count() < 2 {
                observacion.push_str("Nombre no valido - Menos de 2 palabra\n");
            }

            let renombre = Regex::new(r"^[ÑA-Z\s]+$").unwrap();
            if !renombre.is_match(&record.nombre) {
                observacion.push_str("Nombre no valido - Caracteres no validos\n");
            }

            if record.genero != "M" &&
               record.genero != "F" &&
               record.genero != "NULL" {
                observacion.push_str("Genero no valido - Opcion no valida\n");
            }

            if record.estadocivil != "SOLTERO" &&
               record.estadocivil != "CASADO" &&
               record.estadocivil != "DIVORCIADO" &&
               record.estadocivil != "VIUDO" &&
               record.estadocivil != "EN UNION DE HECHO" &&
               record.estadocivil != "NULL" {
                observacion.push_str("Estado Civil no valido - Opcion no valida\n");
            }

            let refecha = Regex::new(r"^\d{4}\-(0?[1-9]|1[012])\-(0?[1-9]|[12][0-9]|3[01])$").unwrap();
            if !refecha.is_match(&record.fechanacimiento) {
                observacion.push_str("Fecha de nacimiento no valida - Formato no valido\n");
            } else {
                let fecha: Vec<char> = record.fechanacimiento.chars().collect();
                let mut edad: String = String::new();
                edad.push(fecha[0]);
                edad.push(fecha[1]);
                edad.push(fecha[2]);
                edad.push(fecha[3]);

                if 2021 - edad.parse::<u32>().unwrap() < 8 || 2021 - edad.parse::<u32>().unwrap() > 95 {
                    observacion.push_str("Fecha de nacimiento no valida - Edad no valida\n");
                }
            }

            if validartelefono(record.telefono.clone()) == false {
                observacion.push_str("Telefono no valido - Formato no valido\n");
            }

            if record.direccion.split_whitespace().count() < 2 {
                observacion.push_str("Direccion no valida - Menos de 2 palabras\n");
            }
            
            let reemail = Regex::new(r"[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?").unwrap();
            if !reemail.is_match(&record.email) {
                observacion.push_str("Email no valido - Formato no valido\n");
            }
            
            if !observacion.is_empty() {
                validado = 0;
            }

            if let Err(err) = insert(record, validado, observacion) {
                println!("{}", err);
                process::exit(1);
            }
        }
    }

    Ok(())
}

fn validarcedula(cedula: String) -> bool{
    let digitos: Vec<char> = cedula.chars().collect();
    let mut provincia: String = String::new();
    let mut digitosnumeros: Vec<u32> = vec![];
    let mut i = 0;
    let mut validador: bool = true;

    for c in digitos.clone(){
        if c.is_alphabetic() {
            validador = false;
            break;
        } else {
            if i < 2 {
                provincia.push(c);
            }
            digitosnumeros.push(c.to_digit(10).unwrap());
            i+=1;
        }
    }

    if validador == false {
        return false;
    } else {
        if digitos.len() != 10 {
            return false
        } else {
            if provincia.parse::<u32>().unwrap() > 24 || 
            provincia.parse::<u32>().unwrap() != 30 || 
            provincia.parse::<u32>().unwrap() != 50 ||
            provincia.parse::<u32>().unwrap() != 80 {
                return false
            } else {
                let valor = digitosnumeros[0]*2 + digitosnumeros[2]*2 + digitosnumeros[4]*2 + digitosnumeros[6]*2 +
                                digitosnumeros[8]*2 + digitosnumeros[1] + digitosnumeros[3] + digitosnumeros[5] + digitosnumeros[7];
                if valor != digitosnumeros[9] {
                    return false;
                } else {
                    return true;
                }        
            }
        }
    }
}

fn validartelefono(telefono: String) -> bool{
    let digitos: Vec<char> = telefono.chars().collect();
    let mut iniciales: String = String::new();
    let mut digitosnumeros: Vec<u32> = vec![];
    let mut i = 0;
    let mut validador: bool = true;

    for c in digitos.clone(){
        if c.is_alphabetic() {
            validador = false;
            break;
        } else {
            if i < 2 {
                iniciales.push(c);
            }
            digitosnumeros.push(c.to_digit(10).unwrap());
            i+=1;
        }
    }

    if validador == false {
        return false;
    } else {
        if digitos.len() < 9 || digitos.len() > 10 {
            return false;
        } else {
            if digitos.len() == 9 {
                if digitosnumeros[0] != 0 {
                    return false;
                } else {
                    if digitosnumeros[1] < 2 || digitosnumeros[1] > 7 {
                        return false;
                    } else {
                        return true;
                    }
                }
            } else {
                if digitosnumeros[0] != 0 {
                    return false;
                } else {
                    if digitosnumeros[1] != 9 {
                        return false;
                    } else {
                        return true;
                    }
                }
            }
        }
    } 
}