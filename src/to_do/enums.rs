use diesel::deserialize::{self, FromSql};
use diesel::expression::AsExpression;
use diesel::serialize::{Output, ToSql};
use diesel::serialize;
use serde::ser::{Serialize, Serializer};
use std::cmp::Eq;
use std::fmt;
use std::io::Write;
use diesel::serialize::IsNull;
use diesel::pg::{Pg, PgValue};
use crate::schema::sql_types::PgStatus;

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = PgStatus)]
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

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "Done" => TaskStatus::Done,
            "Open" => TaskStatus::Open,
            "In Progress" => TaskStatus::InProgress,
            _ => panic!("input {} not supported as at valid status", input_string),
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

impl ToSql<PgStatus, Pg> for TaskStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TaskStatus::Done => out.write_all(b"Done"),
            TaskStatus::Open => out.write_all(b"Open"),
            TaskStatus::InProgress => out.write_all(b"In Progress"),
        }.unwrap_or_else(|_| panic!("Invalid status: {}", self));
        Ok(IsNull::Yes)
    }
}

impl FromSql<PgStatus, Pg> for TaskStatus {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Done" => Ok(TaskStatus::Done),
            b"Open" => Ok(TaskStatus::Open),
            b"In Progress" => Ok(TaskStatus::InProgress),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}



/*


impl Expression for TaskStatus {
    type SqlType = PostgresStatus;
}

impl AsExpression<status> for TaskStatus {
    type Expression = diesel::internal::derives::as_expression::Bound<status, Self>;

    fn as_expression(self) -> Self::Expression {
        diesel::internal::derives::as_expression::Bound::new(self)
    }
}
impl FromSql<PostgresStatus, Postgres> for TaskStatus {
    //fn from_sql(bytes: DB::RawValue<'_>)

    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        let s = String::from_utf8_lossy(bytes.as_bytes());
        match s.as_ref() {
            "Done" => Ok(TaskStatus::Done),
            "Open" => Ok(TaskStatus::Open),
            "InProgress" => Ok(TaskStatus::InProgress),
            _ => Err("Invalid TaskStatus value in database".into()),
        }
    }
}

impl ToSql<status, Postgres> for TaskStatus {
    fn to_sql(&self, out: &mut Output<Postgres>) -> serialize::Result {
        let s = match *self {
            TaskStatus::Done => "Done",
            TaskStatus::Open => "Open",
            TaskStatus::InProgress => "InProgress",
        };
        out.write_all(s.as_bytes())?;
        Ok(IsNull::Yes)
    }
}
*
 */