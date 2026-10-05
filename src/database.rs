use rusqlite::{Connection, Result};

pub fn connect_database() -> Result<Connection> {
    Connection::open("passwords.db")
}

pub fn update_password(
    username: &str,
    website: &str,
    password: &str,
    connection: &Connection,
) -> Result<()> {
    connection.execute(
        "
        UPDATE passwords
        SET password=?3
        WHERE username=?1 AND website=?2
        ",
        [username, website, password],
    )?;
    Ok(())
}

pub fn delete_password(
    username: &str,
    website: &str,
    connection: &Connection,
) -> Result<()> {
    connection.execute(
        "
        DELETE FROM passwords
        WHERE username=?1 AND website=?2
        ",
        [username, website],
    )?;
    Ok(())
}

pub fn store_new_password(
    username: &str,
    website: &str,
    password: &str,
    connection: &Connection,
) -> Result<()> {
    connection.execute(
        "
            INSERT INTO passwords
            (website, username, password)
            VALUES (?1, ?2, ?3)
        ",
        [website, username, password],
    )?;
    Ok(())
}

pub fn get_password(username: &str, website: &str, connection: &Connection) -> Result<String> {
    connection.query_row(
        "
            SELECT password
            FROM passwords
            WHERE website=?1
            AND username=?2;
        ",
        [website, username],
        |row| row.get(0),
    )
}

pub fn initialize_database(connection: &Connection) -> Result<()> {
    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            website TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )
        ",
        [],
    )?;
    Ok(())
}