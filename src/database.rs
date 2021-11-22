use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

use log::{debug, error};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub(crate) struct Image {
    pub id: i32,
    pub filename: String,
    pub f_number: f64,
}

pub(crate) fn get_connection(path: &str) -> Result<Connection, Box<dyn Error>> {
    let path = PathBuf::from_str(path)?;
    let conn = Connection::open(path)?;

    conn.execute(
        "   CREATE TABLE IF NOT EXISTS image
            (
                id       INTEGER PRIMARY KEY,
                filename TEXT NOT NULL,
                f_number REAL
            )",
        [],
    )?;

    Ok(conn)
}

// TODO only insert images where filename does not already exist (maybe add another field for verification (timestamp?))
pub(crate) fn insert_image(conn: &Connection, image: &Image) {
    match conn.execute(
        "INSERT INTO image(filename, f_number) VALUES (?1, ?2)",
        params![&image.filename, &image.f_number],
    ) {
        Ok(_) => debug!("{} was added to the database", &image.filename),
        Err(err) => error!("Error adding {} to the database: {}", &image.filename, err),
    }
}
