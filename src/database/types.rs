use chrono::{DateTime, Utc};
use rusqlite::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SqliteUTC(pub DateTime<Utc>);

// FromSql
impl FromSql for SqliteUTC {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(ts) => {
                let dt = DateTime::<Utc>::from_timestamp(ts, 0).ok_or(FromSqlError::InvalidType)?;
                Ok(SqliteUTC(dt))
            }
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

// ToSql
impl ToSql for SqliteUTC {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.timestamp()))
    }
}

/// Newtype wrapper for Uuid to allow FromSql implementation
#[derive(Debug, Clone)]
pub struct SqliteUuid(pub Uuid);

impl FromSql for SqliteUuid {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(s) => {
                let s = std::str::from_utf8(s)
                    .map_err(|_| rusqlite::types::FromSqlError::InvalidType)?;
                let uuid =
                    Uuid::parse_str(s).map_err(|_| rusqlite::types::FromSqlError::InvalidType)?;
                Ok(SqliteUuid(uuid))
            }
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl ToSql for SqliteUuid {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.to_string()))
    }
}
