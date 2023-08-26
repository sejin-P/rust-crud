use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, put, delete};
use mysql::Pool;
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};

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

#[get("/user/{id}")]
async fn get_user(req: HttpRequest, data: web::Data<Pool>) -> actix_web::Result<impl Responder> {
    let user_id: u64 = req.match_info().get("id").unwrap().parse().unwrap();
    let mut conn = data.get_conn().expect("failed to get connection");

    let user: User = conn.query_map(format!("SELECT name, email, age FROM user WHERE id = {user_id};"), |(name, email, age)| {
        User {
            name,
            email,
            age,
        }
    }).expect("failed to get user").pop().unwrap();

    return Ok(web::Json(user))
}

#[post("/user")]
async fn create_user(web::Json(user_data): web::Json<User>, data: web::Data<Pool>) -> actix_web::Result<impl Responder> {
    let mut conn = data.get_conn().expect("failed to get connection");
    let name = user_data.name;
    let email = user_data.email;
    let age = user_data.age;

    conn.exec_drop(format!("INSERT INTO user (name, email, age) VALUES ('{name}', '{email}', {age});"), ()).expect("failed to insert user");

    return Ok(HttpResponse::NoContent())
}

#[put("/user/{id}")]
async fn update_user(req: HttpRequest, web::Json(user_data): web::Json<User>, data: web::Data<Pool>) -> actix_web::Result<impl Responder> {
    let mut conn = data.get_conn().expect("failed to get connection");

    let user_id: u64 = req.match_info().get("id").unwrap().parse().unwrap();
    let name = user_data.name;
    let age = user_data.age;

    conn.exec_drop(format!("UPDATE user SET name = '{name}', age = {age} WHERE id = {user_id};"), ()).expect("failed to update user");

    return Ok(HttpResponse::NoContent())
}

#[delete("/user/{id}")]
async fn delete_user(req: HttpRequest, data: web::Data<Pool>) -> actix_web::Result<impl Responder> {
    let mut conn = data.get_conn().expect("failed to get connection");

    let user_id: u64 = req.match_info().get("id").unwrap().parse().unwrap();

    conn.exec_drop(format!("DELETE FROM user WHERE id = {user_id};"), ()).expect("failed to delete user");

    return Ok(HttpResponse::NoContent())
}

#[get("/posts/{id}")]
async fn get_post(req: HttpRequest, data: web::Data<Pool>) -> actix_web::Result<impl Responder> {
    let post_id: u64 = req.match_info().get("id").unwrap().parse().unwrap();
    let mut conn = data.get_conn().expect("failed to get connection");

    // TODO query pipelining or something optimization thing
    let post: Post = conn.query_map(format!("SELECT title, body, user_id FROM post WHERE id = {post_id};"), |(title, body, user_id)| {
        Post {
            title,
            body,
            user_id,
        }
    }).expect("failed to get post").pop().unwrap();

    let user: User = conn.query_map(format!("SELECT name, email, age FROM user WHERE id = {};", post.user_id), |(name, email, age)| {
        User {
            name,
            email,
            age,
        }
    }).expect("failed to get user").pop().unwrap();

    let post_user: PostUser = PostUser{
        title: post.title,
        body: post.body,
        user_name: user.name,
        user_email: user.email,
    };

    return Ok(web::Json(post_user))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO make it config var(read from env), dependency injection practically
    let url = "mysql://root:password@localhost:3306/abc";
    let pool = Pool::new(url).expect("failed to create pool");
    let shared_data = web::Data::new(pool);

    // to force the closure to take ownership of `shared_data` (and any other referenced variables), use the `move` keyword
    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(hello)
            .service(echo)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
            .service(get_post)
            .route("/", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
