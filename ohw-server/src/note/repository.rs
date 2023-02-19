use crate::postgres::Database;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct NoteEntity {
    pub id: Uuid,
    pub content: String,
    pub confidential: bool,
}

pub struct CreateNoteInput {
    pub content: String,
    pub confidential: bool,
}

pub trait NoteRepository {
    async fn create(&self, input: CreateNoteInput) -> Result<NoteEntity, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    async fn find(&self, id: Uuid) -> Result<Option<NoteEntity>, sqlx::Error>;
}

pub struct NoteRepositoryImpl {
    database: Database,
}

impl NoteRepositoryImpl {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl NoteRepository for NoteRepositoryImpl {
    async fn create(&self, input: CreateNoteInput) -> Result<NoteEntity, sqlx::Error> {
        let note = sqlx::query_as::<_, NoteEntity>("INSERT INTO notes (id, content, confidential) VALUES (gen_random_uuid(), $1, $2) RETURNING id, content, confidential")
            .bind(&input.content)
            .bind(input.confidential)
            .fetch_one(&self.database)
            .await?;
        Ok(note)
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM notes WHERE id = $1")
            .bind(id)
            .execute(&self.database)
            .await?;
        Ok(())
    }

    async fn find(&self, id: Uuid) -> Result<Option<NoteEntity>, sqlx::Error> {
        let note = sqlx::query_as::<_, NoteEntity>(
            "SELECT id, content, confidential FROM notes WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.database)
        .await?;
        Ok(note)
    }
}

#[cfg(test)]
mod tests {
    use crate::note::repository::{CreateNoteInput, NoteRepository, NoteRepositoryImpl};
    use crate::postgres::create_testcontainers_pool;
    use testcontainers::clients::Cli;
    use testcontainers::images::postgres::Postgres;

    #[tokio::test]
    async fn it_can_create_read_and_delete() -> Result<(), sqlx::Error> {
        let docker = Cli::default();
        let container = docker.run::<Postgres>(Postgres::default());
        let client = create_testcontainers_pool(&container).await?;
        let repository = NoteRepositoryImpl::new(client);
        // First, we attempt to create a new note
        let created = repository
            .create(CreateNoteInput {
                content: "Hello world".to_owned(),
                confidential: true,
            })
            .await;
        assert!(created.is_ok());
        let created = created.unwrap();
        assert_eq!(created.content, "Hello world".to_owned());
        assert_eq!(created.confidential, true);
        // Then we try to locate that note in the database
        let found = repository.find(created.id).await;
        assert!(found.is_ok());
        let found = found.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(created.id, found.id);
        // Then we delete the newly created note
        let deletion = repository.delete(created.id).await;
        assert!(deletion.is_ok());
        // The note should no longer be found
        let missing = repository.find(created.id).await;
        assert!(missing.is_ok());
        let missing = missing.unwrap();
        assert!(missing.is_none());
        Ok(())
    }
}
