use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, Responder, web};
use mysql::Pool;
use mysql::prelude::Queryable;
use crate::errors::user_error::{handle_sql_err, UserError};
use crate::model::user::User;

#[get("/user/{id}")]
async fn get_user(req: HttpRequest, data: web::Data<Pool>) -> actix_web::Result<impl Responder, UserError> {
    let user_id: u64 = req.match_info().get("id").unwrap().parse().map_err(|e| UserError::InternalError)?;
    let mut conn = data.get_conn().map_err(|e| UserError::InternalError)?;

    let user = conn.query_map(format!("SELECT name, email, age FROM user WHERE id = {user_id};"), |(name, email, age)| {
        User {
            name,
            email,
            age,
        }
    }).map_err(handle_sql_err)?.pop().ok_or(UserError::NotFoundError{name: "user", id: user_id}).map_err(|e| e)?;

    return Ok(web::Json(user))
}

#[post("/user")]
async fn create_user(web::Json(user_data): web::Json<User>, data: web::Data<Pool>) -> actix_web::Result<impl Responder, UserError> {
    let mut conn = data.get_conn().map_err(|e| UserError::InternalError)?;
    let name = user_data.name;
    let email = user_data.email;
    let age = user_data.age;

    conn.exec_drop(format!("INSERT INTO user (name, email, age) VALUES ('{name}', '{email}', {age});"), ()).map_err(|e| UserError::InternalError)?;

    return Ok(HttpResponse::Created())
}

#[put("/user/{id}")]
async fn update_user(req: HttpRequest, web::Json(user_data): web::Json<User>, data: web::Data<Pool>) -> actix_web::Result<impl Responder, UserError> {
    let mut conn = data.get_conn().map_err(|e| UserError::InternalError)?;

    let user_id: u64 = req.match_info().get("id").unwrap().parse().map_err(|_| UserError::InternalError)?;
    let name = user_data.name;
    let age = user_data.age;

    conn.exec_drop(format!("UPDATE user SET name = '{name}', age = {age} WHERE id = {user_id};"), ()).map_err(|e| UserError::InternalError)?;

    return Ok(HttpResponse::Ok())
}

#[delete("/user/{id}")]
async fn delete_user(req: HttpRequest, data: web::Data<Pool>) -> actix_web::Result<impl Responder, UserError> {
    let mut conn = data.get_conn().map_err(|e| UserError::InternalError)?;

    let user_id: u64 = req.match_info().get("id").unwrap().parse().map_err(|_| UserError::InternalError)?;

    conn.exec_drop(format!("DELETE FROM user WHERE id = {user_id};"), ()).map_err(|e| UserError::InternalError)?;

    return Ok(HttpResponse::Ok())
}
