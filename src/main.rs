use std::future::Future;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use mysql::Pool;
use mysql::prelude::Queryable;
use mysql::serde::{Deserialize, Serialize};

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
    pub id: u64,
    pub name: String,
    pub email: String,
    pub age: u8,
}

#[get("/user/{id}")]
async fn get_user(req: HttpRequest, data: web::Data<Pool>) -> impl Responder {
    let user_id: u64 = req.match_info().get("id").unwrap().parse().unwrap();
    let mut conn = data.get_conn()?;

    let user: User = conn.query_map("SELECT name, email, age FROM user;", |(name, email, age)| {
        User {
            id: user_id,
            name,
            email,
            age,
        }
    })?.pop().unwrap();

    Ok(web::Json(user))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO make it config var(read from env), dependency injection practically
    let url = "mysql://root:password@localhost:3307/db_name";
    let pool = Pool::new(url)?;

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_user)
            .route("/", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
