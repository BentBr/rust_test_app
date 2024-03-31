use crate::helpers::env::get_int_from_env;
use actix_web::dev::Payload;
use actix_web::error::ErrorServiceUnavailable;
use actix_web::{Error, FromRequest, HttpRequest};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use futures::future::{err, ok, Ready};
use lazy_static::lazy_static;
use std::env;

type PgPool = Pool<ConnectionManager<PgConnection>>;
pub struct DbConnection {
    pub db_connection: PgPool,
}

lazy_static! {
    pub static ref DBCONNECTION: DbConnection = {
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment");
        let max_database_connections = get_int_from_env("MAX_DATABASE_CONNECTIONS".to_string());

        DbConnection {
            db_connection: PgPool::builder()
                .max_size(max_database_connections)
                .build(ConnectionManager::new(database_url))
                .expect("failed to create db connection_pool"),
        }
    };
}

pub struct DB {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl FromRequest for DB {
    type Error = Error;
    type Future = Ready<Result<DB, Error>>;
    fn from_request(_: &HttpRequest, _: &mut Payload) -> Self::Future {
        match DBCONNECTION.db_connection.get() {
            Ok(connection) => ok(DB { connection }),
            Err(error) => {
                sentry::capture_error(&error);

                err(ErrorServiceUnavailable(
                    "could not make connection to database",
                ))
            }
        }
    }
}
