use diesel::r2d2::{PooledConnection, ConnectionManager};
use diesel::result::Error;
use serde_derive::{Deserialize, Serialize};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::{FromSqlRow, AsExpression};
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::deserialize::FromSql;
use diesel::sql_types::Text;
use diesel::backend::RawValue;
use std::io::Write;

use crate::schema::users::{self, dsl::*};

#[derive(Serialize, Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserDTO {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub permission: Permission,
}

impl UserDTO {
    pub fn get_user(uname: String, mut conn: PooledConnection<ConnectionManager<PgConnection>>) -> Result<UserDTO, Error> {
        users.filter(name.eq(uname)).first::<UserDTO>(&mut conn)
    }
}


#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct CreateUserDTO {
    pub name: String,
    pub password: String,
    pub permission: Permission,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Permission {
    Admin,
    Editor,
    User
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