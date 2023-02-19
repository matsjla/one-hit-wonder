use postgres::{Client, Error};

pub fn migrate_000_create_notes(client: &mut Client) -> Result<(), Error> {
    client.execute(
        "CREATE TABLE IF NOT EXISTS notes (id UUID PRIMARY KEY, content TEXT NOT NULL, confidential BOOLEAN NOT NULL);",
        &[]
    )?;
    Ok(())
}
