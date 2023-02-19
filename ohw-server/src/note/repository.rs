use postgres::{Client, Error, Row};
use uuid::Uuid;

pub struct NoteEntity {
    pub id: Uuid,
    pub content: String,
    pub confidential: bool,
}

impl From<&Row> for NoteEntity {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            content: row.get("content"),
            confidential: row.get("confidential"),
        }
    }
}

pub struct CreateNoteInput {
    pub content: String,
    pub confidential: bool,
}

pub trait NoteRepository {
    fn create(&self, client: &mut Client, input: CreateNoteInput) -> Result<NoteEntity, Error>;
    fn delete(&self, client: &mut Client, id: Uuid) -> Result<(), Error>;
    fn find(&self, client: &mut Client, id: Uuid) -> Result<Option<NoteEntity>, Error>;
}

pub struct NoteRepositoryImpl();

impl NoteRepository for NoteRepositoryImpl {
    fn create(&self, client: &mut Client, input: CreateNoteInput) -> Result<NoteEntity, Error> {
        let rows = client.query("INSERT INTO notes (id, content, confidential) VALUES (gen_random_uuid(), $1, $2) RETURNING *", &[
            &input.content,
            &input.confidential,
        ])?;
        let row = rows.first().unwrap();
        Ok(NoteEntity::from(row))
    }

    fn delete(&self, client: &mut Client, id: Uuid) -> Result<(), Error> {
        client.execute("DELETE FROM notes WHERE id = $1", &[&id])?;
        Ok(())
    }

    fn find(&self, client: &mut Client, id: Uuid) -> Result<Option<NoteEntity>, Error> {
        let rows = client.query("SELECT * FROM notes WHERE id = $1", &[&id])?;
        match rows.first() {
            Some(row) => Ok(Some(NoteEntity::from(row))),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::note::repository::{CreateNoteInput, NoteRepository, NoteRepositoryImpl};
    use crate::postgres::create_postgres_connection;
    use postgres::Error;
    use std::env;

    #[test]
    fn it_can_create_read_and_delete() -> Result<(), Error> {
        let database_url = env::var("DATABASE_TEST_URL").unwrap();
        let mut client = create_postgres_connection(database_url, true)?;
        let repository = NoteRepositoryImpl();
        // First, we attempt to create a new note
        let created = repository.create(
            &mut client,
            CreateNoteInput {
                content: "Hello world".to_owned(),
                confidential: true,
            },
        );
        assert!(created.is_ok());
        let created = created.unwrap();
        assert_eq!(created.content, "Hello world".to_owned());
        assert_eq!(created.confidential, true);
        // Then we try to locate that note in the database
        let found = repository.find(&mut client, created.id);
        assert!(found.is_ok());
        let found = found.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(created.id, found.id);
        // Then we delete the newly created note
        let deletion = repository.delete(&mut client, created.id);
        assert!(deletion.is_ok());
        // The note should no longer be found
        let missing = repository.find(&mut client, created.id);
        assert!(missing.is_ok());
        let missing = missing.unwrap();
        assert!(missing.is_none());
        Ok(())
    }
}
