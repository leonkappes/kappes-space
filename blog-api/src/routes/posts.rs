use actix_web::{HttpResponse, Error, web, error};
use diesel::{insert_into, RunQueryDsl};
use serde_json::json;

use crate::model::{CreatePostDTO, PostStatus};
use crate::{model::PostDTO, database::postgres::Pool};
use crate::schema::posts::{dsl::*};

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

pub async fn create_test_data(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let records = vec![
        CreatePostDTO {title: "Test".to_owned() , author: "Leon".to_owned(), published: PostStatus::Published},
        CreatePostDTO {title: "Test23".to_owned() , author: "Nicht Leon".to_owned(), published: PostStatus::Unlisted},
        CreatePostDTO {title: "Private".to_owned() , author: "Nicht Leon".to_owned(), published: PostStatus::Private},
    ];
    let _ = insert_into(posts).values(records).execute(&mut db.get().unwrap());
    Ok(HttpResponse::Ok().json("Created"))
}