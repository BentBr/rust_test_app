use crate::schema::sql_types::Status;
use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize;
use diesel::serialize::IsNull;
use diesel::serialize::{Output, ToSql};
use sentry::Level;
use serde::ser::{Serialize, Serializer};
use std::cmp::Eq;
use std::fmt;
use std::io::Write;

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = Status)]
pub enum TaskStatus {
    Done,
    Open,
    InProgress,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match self {
            Self::Done => "Done".to_string(),
            Self::Open => "Open".to_string(),
            Self::InProgress => "In Progress".to_string(),
        }
    }

    pub fn from_string(input_string: String) -> Option<Self> {
        match input_string.as_str() {
            "Done" => Some(Self::Done),
            "Open" => Some(Self::Open),
            "In Progress" => Some(Self::InProgress),
            _ => {
                sentry::capture_message(
                    format!("input '{}' not supported as at valid status", input_string).as_str(),
                    Level::Error,
                );
                None
            }
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Done => {
                write!(f, "Done")
            }
            Self::Open => {
                write!(f, "Open")
            }
            Self::InProgress => {
                write!(f, "In Progress")
            }
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.stringify().as_str())
    }
}

impl PartialEq for TaskStatus {
    fn eq(&self, other: &Self) -> bool {
        self.stringify() == other.stringify()
    }
}
impl Eq for TaskStatus {}

impl ToSql<Status, Pg> for TaskStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TaskStatus::Done => out.write_all(b"Done"),
            TaskStatus::Open => out.write_all(b"Open"),
            TaskStatus::InProgress => out.write_all(b"In progress"),
        }
        .unwrap_or_else(|_| panic!("Invalid status: {}", self));
        Ok(IsNull::No)
    }
}

impl FromSql<Status, Pg> for TaskStatus {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Done" => Ok(TaskStatus::Done),
            b"Open" => Ok(TaskStatus::Open),
            b"In progress" => Ok(TaskStatus::InProgress),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
