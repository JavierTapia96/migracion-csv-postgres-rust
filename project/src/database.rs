use postgres::{Client, NoTls, Error};

use crate::reader::Persona;

pub fn database() -> Result<(), Error>{
    let mut client = Client::connect("postgresql://migracion:migracion@localhost/migracion", NoTls)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS persona (
        id      SERIAL PRIMARY KEY,
        identificacion    VARCHAR ( 255 ) UNIQUE NOT NULL,
        nombre    VARCHAR ( 255 ) NOT NULL,
        genero    VARCHAR ( 255 ), 
        estadocivil    VARCHAR ( 255 ),
        fechanacimiento    VARCHAR ( 255 ),
        telefono     VARCHAR ( 255 ) UNIQUE NOT NULL,
        direccion    VARCHAR( 255 ),
        email    VARCHAR ( 255 ) UNIQUE NOT NULL,
        validado    INT,
        observacion    VARCHAR( 1024 )
        )
    ")?;

    Ok(())
}

pub fn insert(persona: Persona, validado: i32, observacion: String) -> Result<(), Error>{
    let mut client = Client::connect("postgresql://migracion:migracion@localhost/migracion", NoTls)?;

    client.execute(
        "INSERT INTO persona (identificacion, nombre, genero, estadocivil, fechanacimiento, telefono, direccion, email, validado, observacion) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        &[&persona.identificacion, &persona.nombre, &persona.genero, &persona.estadocivil, &persona.fechanacimiento, &persona.telefono, &persona.direccion, &persona.email, &validado, &observacion],
    )?;

    Ok(())
}