use actix_web::http::StatusCode;
use std::fmt;

#[derive(Debug)]
pub struct Error(anyhow::Error);
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<T> for Error
where
    anyhow::Error: From<T>,
{
    fn from(error: T) -> Self {
        Error(anyhow::Error::from(error))
    }
}
impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        use diesel::result::Error;
        match self.0.downcast_ref::<Error>() {
            Some(Error::NotFound) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
pub type JsResult<T> = Result<actix_web::web::Json<T>>;
