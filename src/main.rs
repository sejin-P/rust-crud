use std::fmt::Display;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, http::{header::ContentType, StatusCode}, put, delete, error};
use actix_web::middleware::Logger;
use mysql::{Pool};
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};
use derive_more::{Display, Error};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// TODO refactoring user model, api
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct User {
    pub name: String,
    #[serde(default)]
    pub email: String,
    pub age: u8,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub user_id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PostUser {
    pub title: String,
    pub body: String,
    pub user_name: String,
    pub user_email: String,
}


#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    #[display(fmt = "Could not find data of {}, id {}.", name, id)]
    NotFoundError {name: &'static str, id: u64 },
    #[display(fmt = "Invalid request.")]
    ValidationError,
    #[display(fmt = "Unauthorized.")]
    UnauthorizedError,
}

impl error::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::NotFoundError{name, id} => StatusCode::NOT_FOUND,
            UserError::ValidationError => StatusCode::BAD_REQUEST,
            UserError::UnauthorizedError => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

fn handle_sql_err(e: mysql::Error) -> UserError {
    return UserError::InternalError
}

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

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    // TODO make it config var(read from env), dependency injection practically
    let url = "mysql://root:password@localhost:3306/abc";
    let pool = Pool::new(url).expect("failed to create pool");
    let shared_data = web::Data::new(pool);

    // to force the closure to take ownership of `shared_data` (and any other referenced variables), use the `move` keyword
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(shared_data.clone())
            .service(hello)
            .service(echo)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
            .service(get_post)
            .service(create_post)
            .service(update_post)
            .service(delete_post)
            .route("/", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
