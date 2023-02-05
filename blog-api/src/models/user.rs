use crate::auth::hash_password;
use crate::schema::users::{self, dsl::*};
use actix_web::error;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::backend::RawValue;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{insert_into, prelude::*};
use diesel::{AsExpression, FromSqlRow};
use serde_derive::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserDTO {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub permission: Permission,
}

impl UserDTO {
    pub fn get_user(
        uname: String,
        mut conn: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<UserDTO, Error> {
        users.filter(name.eq(uname)).first::<UserDTO>(&mut conn)
    }

    pub fn login(&self, provided_password: String) -> Result<bool, argon2::password_hash::Error> {
        // Verify password against PHC string.
        // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
        // `Argon2` instance.
        let parsed_hash = match PasswordHash::new(&self.password) {
            Ok(it) => it,
            Err(err) => return Err(err),
        };
        Ok(Argon2::default()
            .verify_password(provided_password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct CreateUserDTO {
    pub name: String,
    pub password: String,
    pub permission: Permission,
}

impl CreateUserDTO {
    // TODO: Get the hashing into utility function
    pub fn new_user(
        uname: String,
        upassword: String,
        upermission: Permission,
        mut conn: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<UserDTO, actix_web::Error> {
        let password_hash = match hash_password(upassword) {
            Ok(hash) => hash,
            Err(err) => return Err(error::ErrorInternalServerError(err)),
        };

        let new_user = CreateUserDTO {
            name: uname,
            password: password_hash,
            permission: upermission,
        };

        let res = insert_into(users)
            .values(new_user)
            .get_result::<UserDTO>(&mut conn);

        match res {
            Ok(user) => return Ok(user),
            Err(err) => return Err(error::ErrorBadRequest(err)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Permission {
    Admin,
    Editor,
    User,
}

impl ToSql<Text, Pg> for Permission {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            Permission::Admin => out.write_all(b"ADMIN")?,
            Permission::Editor => out.write_all(b"Editor")?,
            Permission::User => out.write_all(b"User")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for Permission {
    fn from_sql(bytes: RawValue<'_, Pg>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"ADMIN" => Ok(Permission::Admin),
            b"Editor" => Ok(Permission::Editor),
            b"User" => Ok(Permission::User),
            _ => Err("Unrecognized enum".into()),
        }
    }
}
