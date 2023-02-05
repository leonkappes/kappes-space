use actix_web::{
    error,
    web::{self, Data, ReqData},
    Error, HttpResponse,
};
use diesel::{insert_into, BelongingToDsl, RunQueryDsl};
use serde::Deserialize;
use serde_json::json;

use crate::{
    auth::UserToken,
    database::postgres::Pool,
    models::{
        post::{CreatePostDTO, PostDTO, PostStatus},
        user::{CreateUserDTO, Permission, UserDTO},
    },
    schema::{posts::dsl::*, users::dsl::*},
};

pub async fn get_posts(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let posts_res = PostDTO::get_public_posts(db.get().unwrap());
    if let Ok(posts_data) = posts_res {
        return Ok(HttpResponse::Ok().json(json!({
            "posts": posts_data,
            "count": posts_data.len(),
            "message": "Successfully retrieved posts"
        })));
    }
    Err(error::ErrorNotFound("No posts found"))
}

pub async fn get_by_user(
    db: web::Data<Pool>,
    uname: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user = UserDTO::get_user(uname.to_owned(), db.get().unwrap());
    if let Ok(user) = user {
        let post_data: Result<Vec<PostDTO>, _> =
            PostDTO::belonging_to(&user).load::<PostDTO>(&mut db.get().unwrap());
        if let Ok(post_data) = post_data {
            return Ok(HttpResponse::Ok().json(json!({
                "posts": post_data,
                "message": "Successfully retrieved posts"
            })));
        }
    }
    Err(error::ErrorNotFound("No posts found"))
}

#[derive(Deserialize)]
pub struct FormData {
    text: String,
    text_md: String,
    status: PostStatus,
    title: String,
}

pub async fn insert_post(
    db: web::Data<Pool>,
    creds: Option<ReqData<UserToken>>,
    form: web::Form<FormData>,
) -> Result<HttpResponse, Error> {
    let user = if let Some(creds) = creds {
        match UserDTO::get_user_by_id(creds.user_id, db.get().unwrap()) {
            Ok(it) => it,
            Err(_) => {
                return Err(error::ErrorForbidden(json!({
                    "message": "No permission to create a post1"
                })))
            }
        }
    } else {
        return Err(error::ErrorForbidden(json!({
            "message": "No permission to create a post2"
        })));
    };

    if user.permission == Permission::User {
        return Err(error::ErrorForbidden(json!({
            "message": "No permission to create a post3"
        })));
    }

    let post = CreatePostDTO {
        title: form.title.to_owned(),
        author: user.id,
        published: form.status,
        content: form.text.to_owned(),
        content_md: form.text_md.to_owned(),
    };

    match insert_into(posts)
        .values(post)
        .get_result::<PostDTO>(&mut db.get().unwrap())
    {
        Ok(res) => {
            return Ok(HttpResponse::Ok().json(json!({
                "message": "Post created successfully",
                "post": res
            })))
        }
        Err(err) => {
            return Err(error::ErrorInternalServerError(json!({
                "message": "No permission to create a post",
                "error": err.to_string()
            })))
        }
    }
}

pub async fn create_test_data(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let record_users = vec![CreateUserDTO {
        name: "Leon".to_owned(),
        password: "1234".to_owned(),
        permission: crate::models::user::Permission::Admin,
    }];
    let records = vec![
        CreatePostDTO {
            title: "Test".to_owned(),
            content: "123".to_owned(),
            content_md: "23123".to_owned(),
            author: 1,
            published: PostStatus::Published,
        },
        CreatePostDTO {
            title: "Test23".to_owned(),
            content: "123".to_owned(),
            content_md: "23123".to_owned(),
            author: 1,
            published: PostStatus::Unlisted,
        },
        CreatePostDTO {
            title: "Private".to_owned(),
            content: "123".to_owned(),
            content_md: "23123".to_owned(),
            author: 1,
            published: PostStatus::Private,
        },
    ];
    let _ = insert_into(users)
        .values(record_users)
        .execute(&mut db.get().unwrap());
    let _ = insert_into(posts)
        .values(records)
        .execute(&mut db.get().unwrap());
    Ok(HttpResponse::Ok().json("Created"))
}
