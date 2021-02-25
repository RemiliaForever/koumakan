use actix_web::ResponseError;
use http::StatusCode;

#[derive(Debug)]
pub struct ResError {
    code: Option<http::StatusCode>,
    cause: Box<dyn std::fmt::Debug>,
}

impl std::fmt::Display for ResError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl ResError {
    pub fn new<T>(e: T) -> ResError
    where
        T: std::fmt::Debug + 'static,
    {
        ResError {
            code: None,
            cause: Box::new(e),
        }
    }
}

impl From<http::StatusCode> for ResError {
    fn from(code: http::StatusCode) -> ResError {
        ResError {
            code: Some(code),
            cause: Box::new(""),
        }
    }
}

impl From<sqlx::Error> for ResError {
    fn from(err: sqlx::Error) -> ResError {
        ResError {
            code: Some(http::StatusCode::INTERNAL_SERVER_ERROR),
            cause: Box::new(err),
        }
    }
}

impl<T> From<Box<T>> for ResError
where
    T: std::fmt::Debug + 'static,
{
    fn from(err: Box<T>) -> ResError {
        ResError {
            code: Some(http::StatusCode::INTERNAL_SERVER_ERROR),
            cause: err,
        }
    }
}

impl ResponseError for ResError {
    #[inline]
    fn status_code(&self) -> StatusCode {
        match self.code {
            Some(code) => code,
            None => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
