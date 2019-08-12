use actix_web::{error, http, HttpResponse};
use failure::Fail;

#[derive(Fail, Debug)]
pub enum UserError {
    #[fail(display = "User error, please check your input")]
    InputError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UserError::InputError => {
                HttpResponse::new(http::StatusCode::OK)
            }
        }
    }
}


