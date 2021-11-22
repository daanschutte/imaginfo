use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

use log::{debug, error};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub(crate) struct SonyImage {
    pub id: i32,
    pub filename: String,
    pub f_number: f64,
}

pub(crate) fn get_connection(path: &str) -> Result<Connection, Box<dyn Error>> {
    let path = PathBuf::from_str(path)?;
    let conn = Connection::open(path)?;

    // TODO how do we run migrations? (probably rebuild the db?
    //      this will cause us to lose data on images that are no longer present)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sony_arw
            (
                id              INTEGER PRIMARY KEY,
                filename        TEXT,
                f_number        REAL
            )",
        [],
    )?;

    Ok(conn)
}

// TODO only add image if it is unique - use datetime?
pub(crate) fn insert_sony(conn: &Connection, image: &SonyImage) {
    match conn.execute(
        "INSERT INTO sony_arw(filename, f_number) VALUES (?1, ?2)",
        params![&image.filename, &image.f_number],
    ) {
        Ok(_) => debug!("{} was added to the database", &image.filename),
        Err(err) => error!("Error adding {} to the database: {}", &image.filename, err),
    }
}
