use std::fmt;

use actix_web::error::BlockingError;
use actix_web::web::HttpResponse;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::{DatabaseError, NotFound};
use diesel::result::Error;
use actix_web::error::PayloadError::Http2Payload;

#[derive(Debug)]
pub enum AppError {
    RecordAlreadyExist,
    RecordNotFound,
    DatabaseError(diesel::result::Error),
    OperationCanceled,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    err: String,
}


impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::RecordAlreadyExist => write!(f, "This recird violates a unique contraint"),
            AppError::RecordNotFound => write!(f, "This record does not exist"),
            AppError::DatabaseError(e) => write!(f, "Database Error: {:?}", e),
            AppError::OperationCanceled => write!(f, "Running operation was cancelled"),
        }
    }
}


impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExist,
            NotFound => AppError::RecordNotFound,
            _ => AppError::DatabaseError(e)
        }
    }
}

impl From<BlockingError<AppError>> for AppError {
    fn from(e: BlockingError<AppError>) -> Self {
        match e {
            BlockingError::Error(inner) => inner,
            BlockingError::Canceled => AppError::OperationCanceled
        }
    }
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let err = format!("{}", self);
        let mut builder = match self {
            AppError::RecordAlreadyExist => HttpResponse::BadRequest(),
            AppError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError()
        };
        builder.json(ErrorResponse { err })
    }

    fn render_response(&self) -> HttpResponse {
        self.error_response()
    }
}
