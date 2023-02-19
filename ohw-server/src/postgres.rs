use crate::migrations::migrate_000_create_notes;
use openssl::ssl::{SslConnector, SslMethod};
use postgres::{Client, Error};
use postgres_openssl::MakeTlsConnector;

/// Initializes a connection to the PostGreSQL-compatible database, running all of the
/// migrations against the database if specified.
pub fn create_postgres_connection(
    database_url: String,
    run_migrations: bool,
) -> Result<Client, Error> {
    let ssl_connector = SslConnector::builder(SslMethod::tls()).unwrap();
    let tls_connector = MakeTlsConnector::new(ssl_connector.build());
    let mut client = Client::connect(&database_url, tls_connector)?;
    if run_migrations {
        run_postgres_migrations(&mut client)?;
    }
    Ok(client)
}

fn run_postgres_migrations(client: &mut Client) -> Result<(), Error> {
    migrate_000_create_notes(client)?;
    Ok(())
}
