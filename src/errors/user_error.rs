use actix_web::{error, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum UserError {
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

pub fn handle_sql_err(e: mysql::Error) -> UserError {
    return UserError::InternalError
}