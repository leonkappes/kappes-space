use serde_derive::{Deserialize, Serialize};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::{FromSqlRow, AsExpression};
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::deserialize::FromSql;
use diesel::sql_types::Text;
use std::io::Write;
use diesel::backend::RawValue;
use crate::schema::posts;

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct PostDTO {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub published: PostStatus,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = posts)]
pub struct CreatePostDTO {
    pub title: String,
    pub author: String,
    pub published: PostStatus,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum PostStatus {
    Published,
    Private,
    Unlisted
}

impl ToSql<Text, Pg> for PostStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            PostStatus::Private => out.write_all(b"PRIVATE")?,
            PostStatus::Published => out.write_all(b"PUBLISHED")?,
            PostStatus::Unlisted => out.write_all(b"UNLISTED")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for PostStatus {
    fn from_sql(bytes: RawValue<'_, Pg>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"PRIVATE" => Ok(PostStatus::Private),
            b"PUBLISHED" => Ok(PostStatus::Published),
            b"UNLISTED" => Ok(PostStatus::Unlisted),
            _ => Err("Unrecognized enum".into()),
        }
    }
}