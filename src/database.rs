use std::error::Error;

use log::{debug, error};
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Image {
    id: i32,
    filename: String,
    f_number: f64,
}

fn init_db(path: &str) -> Connection {
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

    conn
}

pub(crate) fn get_connection(path: &str) -> Result<(), Box<dyn Error>> {
    let conn = init_db(path);

    //
    // let me = Person {
    //     id: 0,
    //     name: "Steven".to_string(),
    //     data: None,
    // };
    // conn.execute(
    //     "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //     params![me.name, me.data],
    // )?;
    //
    // let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    // let person_iter = stmt.query_map([], |row| {
    //     Ok(Person {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //         data: row.get(2)?,
    //     })
    // })?;
    //
    // for person in person_iter {
    //     println!("Found person {:?}", person.unwrap());
    // }

    let i = Image {
        id: -1,
        filename: String::from("test_image2.ARW"),
        f_number: 4.2,
    };

    insert_image(&conn, &i);

    Ok(())
}

fn insert_image(conn: &Connection, image: &Image) {
    match conn.execute(
        "INSERT INTO image(filename, f_number) VALUES (?1, ?2)",
        params![&image.filename, &image.f_number],
    ) {
        Ok(updated) => debug!("{} was added to the database", &image.filename),
        Err(err) => error!("Error adding {} to the database: {}", &image.filename, err),
    }
}
