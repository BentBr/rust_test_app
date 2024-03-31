use crate::database::DB;
use crate::schema::users;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Identifiable, QueryDsl, Queryable, RunQueryDsl};
use uuid::Uuid;

#[derive(Queryable, Clone, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub salutation: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub creation_date: NaiveDateTime,
    pub modification_date: Option<NaiveDateTime>,
    pub deletion_date: Option<NaiveDateTime>,
}

#[derive(Queryable, Clone, Identifiable)]
#[diesel(table_name = users)]
pub struct PasswordUser {
    pub id: i32,
    pub uuid: Uuid,
    pub password: String,
}

impl User {
    pub fn verify(&self, password: String) -> bool {
        verify(password.as_str(), &self.password).unwrap()
    }
}

pub fn fetch_item(uuid: Uuid, mut db: DB) -> Vec<User> {
    // Loading it from DB
    users::table
        .filter(users::columns::uuid.eq(uuid))
        .order(users::columns::id.asc())
        .load::<User>(&mut db.connection)
        .unwrap()
}

pub fn delete_item(uuid: Uuid, mut db: DB) -> Option<Uuid> {
    match diesel::delete(users::table.filter(users::columns::uuid.eq(uuid)))
        .execute(&mut db.connection)
    {
        Ok(exec) => {
            // Verbosity for console
            if exec > 0 {
                return Some(uuid);
            }

            None
        }
        Err(error) => {
            // Logging a bit
            sentry::capture_error(&error);

            None
        }
    }
}

pub fn edit_item(
    uuid: Uuid,
    username: String,
    email: String,
    salutation: String,
    first_name: String,
    last_name: String,
    mut db: DB,
) -> Vec<User> {
    let results = users::table.filter(users::columns::uuid.eq(&uuid));
    let exec = diesel::update(results)
        .set((
            users::columns::username.eq(username),
            users::columns::email.eq(email),
            users::columns::salutation.eq(salutation),
            users::columns::first_name.eq(first_name),
            users::columns::last_name.eq(last_name),
        ))
        .execute(&mut db.connection);

    if let Err(error) = exec {
        sentry::capture_error(&error);
    }

    fetch_item(uuid, db)
}

pub fn update_password(
    uuid: Uuid,
    old_password: String,
    new_password: String,
    db: DB,
    mut db2: DB,
) -> Option<User> {
    // Fetch the user to verify existence
    let user = fetch_item(uuid, db);
    if let Some(user) = user.first() {
        // Check if old password fits
        if !user.verify(old_password) {
            return None;
        }

        // Hash the new password
        let hashed_password = match hash(new_password.as_str(), DEFAULT_COST) {
            Ok(hash) => hash,
            Err(error) => {
                sentry::capture_error(&error);

                return None;
            }
        };

        // Prepare update statement
        let results = users::table.filter(users::columns::uuid.eq(&uuid));
        let exec = diesel::update(results)
            .set((users::columns::password.eq(hashed_password),))
            .execute(&mut db2.connection);

        if let Err(error) = exec {
            sentry::capture_error(&error);
            return None;
        }

        // Re-fetch the updated user
        Some(user.clone())
    } else {
        None
    }
}

pub fn fetch_user_by_login(email: String, password: String, mut db: DB) -> Option<User> {
    let users = users::table
        .filter(users::columns::email.eq(email))
        .load::<User>(&mut db.connection);

    match users {
        Ok(users) => {
            if users.len() != 1 {
                return None;
            }

            match users.first() {
                None => None,
                Some(user) => {
                    if user.verify(password) {
                        return Some(user.clone());
                    }

                    None
                }
            }
        }
        Err(error) => {
            sentry::capture_error(&error);

            None
        }
    }
}
