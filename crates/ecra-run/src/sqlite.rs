use std::path::Path;

use rusqlite::{Connection, ErrorCode};

use crate::{RunError, RunErrorCategory, RunErrorCode};

const SQLITE_SYNCHRONOUS_FULL: i64 = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SqliteConfiguration {
    journal_mode: String,
    synchronous: i64,
    foreign_keys: bool,
    trusted_schema: bool,
}

impl SqliteConfiguration {
    #[must_use]
    pub fn journal_mode(&self) -> &str {
        &self.journal_mode
    }

    #[must_use]
    pub const fn synchronous(&self) -> i64 {
        self.synchronous
    }

    #[must_use]
    pub const fn foreign_keys(&self) -> bool {
        self.foreign_keys
    }

    #[must_use]
    pub const fn trusted_schema(&self) -> bool {
        self.trusted_schema
    }
}

pub(crate) fn open_configured(path: impl AsRef<Path>) -> Result<Connection, RunError> {
    let connection =
        Connection::open(path).map_err(|error| map_sqlite("open SQLite store", error))?;
    connection
        .pragma_update(None, "journal_mode", "WAL")
        .map_err(|error| map_sqlite("set journal_mode=WAL", error))?;
    connection
        .pragma_update(None, "synchronous", "FULL")
        .map_err(|error| map_sqlite("set synchronous=FULL", error))?;
    connection
        .pragma_update(None, "foreign_keys", true)
        .map_err(|error| map_sqlite("set foreign_keys=ON", error))?;
    connection
        .pragma_update(None, "trusted_schema", false)
        .map_err(|error| map_sqlite("set trusted_schema=OFF", error))?;

    let configuration = read_configuration(&connection)?;
    if !configuration.journal_mode.eq_ignore_ascii_case("wal")
        || configuration.synchronous != SQLITE_SYNCHRONOUS_FULL
        || !configuration.foreign_keys
        || configuration.trusted_schema
    {
        return Err(RunError::new(
            RunErrorCategory::Storage,
            RunErrorCode::StoreConfigurationInvalid,
            format!(
                "SQLite configuration mismatch: journal_mode={}, synchronous={}, foreign_keys={}, trusted_schema={}",
                configuration.journal_mode,
                configuration.synchronous,
                configuration.foreign_keys,
                configuration.trusted_schema
            ),
        ));
    }

    Ok(connection)
}

pub(crate) fn read_configuration(connection: &Connection) -> Result<SqliteConfiguration, RunError> {
    let journal_mode: String = connection
        .pragma_query_value(None, "journal_mode", |row| row.get(0))
        .map_err(|error| map_sqlite("read journal_mode", error))?;
    let synchronous: i64 = connection
        .pragma_query_value(None, "synchronous", |row| row.get(0))
        .map_err(|error| map_sqlite("read synchronous", error))?;
    let foreign_keys: i64 = connection
        .pragma_query_value(None, "foreign_keys", |row| row.get(0))
        .map_err(|error| map_sqlite("read foreign_keys", error))?;
    let trusted_schema: i64 = connection
        .pragma_query_value(None, "trusted_schema", |row| row.get(0))
        .map_err(|error| map_sqlite("read trusted_schema", error))?;

    Ok(SqliteConfiguration {
        journal_mode,
        synchronous,
        foreign_keys: foreign_keys == 1,
        trusted_schema: trusted_schema != 0,
    })
}

pub(crate) fn map_sqlite(context: &str, error: rusqlite::Error) -> RunError {
    let code = match error.sqlite_error_code() {
        Some(ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked) => RunErrorCode::StoreBusy,
        _ => RunErrorCode::StorageError,
    };
    RunError::new(
        RunErrorCategory::Storage,
        code,
        format!("{context}: {error}"),
    )
}
