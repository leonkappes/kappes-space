use actix_web::{HttpResponse, Error, web, error};
use diesel::{insert_into, RunQueryDsl, BelongingToDsl};
use serde::Deserialize;
use serde_json::json;

use crate::{schema::{posts::{dsl::*, self}, users::{dsl::*}}, models::{post::{PostDTO, CreatePostDTO, PostStatus}, user::{CreateUserDTO, UserDTO}}, database::postgres::Pool};

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

pub async fn get_by_user(db: web::Data<Pool>, uname: web::Path<String>) -> Result<HttpResponse, Error> {
    let user = UserDTO::get_user(uname.to_owned(), db.get().unwrap());
    if let Ok(user) = user {
        let post_data: Result<Vec<PostDTO>, _> = PostDTO::belonging_to(&user).load::<PostDTO>(&mut db.get().unwrap());
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

pub async fn insert_post(db: web::Data<Pool>, form: web::Form<FormData>) -> Result<HttpResponse, Error> {

    let post = CreatePostDTO {
        title: form.title.to_owned(),
        published: form.status,
        content: form.text.to_owned(),
        content_md: form.text_md.to_owned(),
        author: 1,
    };

    let res = insert_into(posts).values(post).execute(&mut db.get().unwrap());

    match res {
        Ok(_) => return Ok(HttpResponse::Ok().json("value")),
        Err(err) => return Err(error::ErrorBadRequest(err))
    }
}

pub async fn create_test_data(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let record_users = vec![
        CreateUserDTO {name: "Leon".to_owned(), password: "1234".to_owned(), permission: crate::models::user::Permission::Admin}
    ];
    let records = vec![
        CreatePostDTO {title: "Test".to_owned() , content: "123".to_owned(), content_md: "23123".to_owned(), author: 1, published: PostStatus::Published},
        CreatePostDTO {title: "Test23".to_owned() , content: "123".to_owned(), content_md: "23123".to_owned(), author: 1, published: PostStatus::Unlisted},
        CreatePostDTO {title: "Private".to_owned() , content: "123".to_owned(), content_md: "23123".to_owned(), author: 1, published: PostStatus::Private},
    ];
    let _ = insert_into(users).values(record_users).execute(&mut db.get().unwrap());
    let _ = insert_into(posts).values(records).execute(&mut db.get().unwrap());
    Ok(HttpResponse::Ok().json("Created"))
}