use dirs::data_local_dir;
use rusqlite::{Connection, Error, Result};
use std::path::PathBuf;

fn db_path() -> Result<PathBuf, Error> {
    match data_local_dir() {
        Some(dir) => Ok(dir.join("page-vault/page-vault.db")),
        None => {
            eprintln!("Failed to locate data directory");
            return Err(Error::InvalidPath("Could not find data directory".into()));
        }
    }
}

pub fn db_init() -> Result<()> {
    let conn = match Connection::open(db_path()?) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return Err(e);
        }
    };

    if let Err(e) = conn.execute(
        "CREATE TABLE IF NOT EXISTS page (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            html TEXT NOT NULL,
            url TEXT NOT NULL,
            scraped_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    ) {
        eprintln!("Failed to initialize database schema: {}", e);
        return Err(e);
    }

    Ok(())
}

pub fn insert_page(url: &str, html: &str) -> Result<()> {
    let conn = match Connection::open(db_path()?) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to open connection to database: {}", e);
            return Err(e);
        }
    };

    if let Err(e) = conn.execute("INSERT INTO page (html, url) VALUES (?1, ?2)", (html, url)) {
        eprintln!("Failed to insert into database: {}", e);
        return Err(e);
    } else {
        println!("Saved page to database!")
    }

    Ok(())
}
