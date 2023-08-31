extern crate rusqlite;

use rusqlite::{params, Connection, Result};

#[derive(Debug)]

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn crate_db() -> Result<Connection> {
    let database_file = "data.db";
    let conn = Connection::open(database_file)?;
    let _ = conn.execute("DROP TABLE person", []);

    conn.execute(
        "CREATE TABLE person(
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        data BLOB
    )",
        [],
    )?;

    Ok(conn)
}

fn inser_data(conn: &Connection) -> Result<()> {
    let p1 = Person {
        id: 1,
        name: "yts".to_string(),
        data: None,
    };
    let p2 = Person {
        id: 2,
        name: "yang".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (id, name, data) VALUES (?1, ?2, ?3), (?4, ?5, ?6);",
        params![p1.id, p1.name, p1.data, p2.id, p2.name, p2.data],
    )?;
    Ok(())
}

fn get_data(conn: &Connection)-> Result<Vec<Person>>{
    let mut stmt = conn.prepare("SELECT id, name, data from person")?;
    let person_iterator = stmt.query_map([], |row|{
        Ok(Person{
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;
    let mut persons = Vec::new();
    for p in person_iterator{
        persons.push(p?);
    }
    Ok(persons)
}

fn main()->Result<()>{
    let conn = crate_db()?;
    inser_data(&conn)?;
    let persons= get_data(&conn)?;
    for p in persons {
        println!("{:?}", p)
    }
    Ok(())
}
