use rusqlite::{Connection, Result};

pub fn db_init() -> Result<()> {
    let conn = Connection::open("~/.local/share/page-vault/page-vault.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS page (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            html TEXT NOT NULL,
            url TEXT NOT NULL,
            scraped_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )?;
    Ok(())
}

pub fn insert_page(url: &str, html: &str) -> Result<()> {
    let conn = Connection::open("page-vault.db")?;

    conn.execute(
        "INSERT INTO page (html, url)
        VALUES (?1, ?2)",
        (url, html),
    )?;

    Ok(())
}
