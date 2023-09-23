use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, Responder, web};
use mysql::Pool;
use mysql::prelude::Queryable;
use crate::errors::user_error::{handle_sql_err, UserError};
use crate::model::post::Post;
use crate::model::post_user::PostUser;
use crate::model::user::User;

#[get("/posts/{id}")]
async fn get_post(req: HttpRequest, data: web::Data<Pool>) -> actix_web::Result<impl Responder, UserError> {
    let post_id: u64 = req.match_info().get("id").unwrap().parse().map_err(|e| UserError::InternalError)?;
    let mut conn = data.get_conn().map_err(|e| UserError::InternalError)?;

    // TODO query pipelining or something optimization thing
    let post = conn.query_map(format!("SELECT title, body, user_id FROM post WHERE id = {post_id};"), |(title, body, user_id)| {
        Post {
            title,
            body,
            user_id,
        }
    }).map_err(handle_sql_err)?.pop().ok_or(UserError::NotFoundError{name: "post", id: post_id}).map_err(|e| e)?;

    let user: User = conn.query_map(format!("SELECT name, email, age FROM user WHERE id = {};", post.user_id), |(name, email, age)| {
        User {
            name,
            email,
            age,
        }
    }).map_err(handle_sql_err)?.pop().ok_or(UserError::NotFoundError{name: "user", id: post.user_id}).map_err(|e| e)?;

    let post_user: PostUser = PostUser{
        title: post.title,
        body: post.body,
        user_name: user.name,
        user_email: user.email,
    };

    return Ok(web::Json(post_user))
}

#[post("/posts")]
async fn create_post(web::Json(post_data): web::Json<Post>, data: web::Data<Pool>) -> actix_web::Result<impl Responder, UserError> {
    let mut conn = data.get_conn().map_err(|e| UserError::InternalError)?;

    conn.exec_drop(format!("INSERT INTO post (title, body, user_id) VALUES ('{}', '{}', '{}');", post_data.title, post_data.body, post_data.user_id), ()).map_err(|_| UserError::InternalError)?;

    return Ok(HttpResponse::Created())
}

// TODO add auth especially
#[put("/posts/{id}")]
async fn update_post(req: HttpRequest, web::Json(post_data): web::Json<Post>, data: web::Data<Pool>) -> actix_web::Result<impl Responder, UserError> {
    let mut conn = data.get_conn().map_err(|e| UserError::InternalError)?;

    let post_id: u64 = req.match_info().get("id").unwrap().parse::<u64>().map_err(|e| UserError::InternalError)?;

    conn.exec_drop(format!("UPDATE post SET title = '{}', body = '{}' WHERE id = {};", post_data.title, post_data.body, post_id), ()).map_err(|_| UserError::InternalError)?;
    return Ok(HttpResponse::Ok())
}

#[delete("/posts/{id}")]
async fn delete_post(req: HttpRequest, data: web::Data<Pool>) -> actix_web::Result<impl Responder, UserError> {
    let mut conn = data.get_conn().map_err(|e| UserError::InternalError)?;

    let post_id: u64 = req.match_info().get("id").unwrap().parse().map_err(|e| UserError::InternalError)?;

    conn.exec_drop(format!("DELETE FROM post WHERE id = {post_id};"), ()).map_err(|_| UserError::InternalError)?;

    return Ok(HttpResponse::Ok())
}