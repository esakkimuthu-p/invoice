use std::{error, fmt, result};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorKind {
    Internal,
    InvalidData,
    NotFound,
    LogicalError,
    UnAuthorized,
    UnInitializedConnectionManager,
    BadInput(mongodb::bson::oid::Error),
    DatabaseError(mongodb::error::Error),
    DataIntergrityError(mongodb::bson::de::Error),
    DataParseError(mongodb::bson::ser::Error),
}

impl From<mongodb::error::Error> for ErrorKind {
    fn from(err: mongodb::error::Error) -> Self {
        ErrorKind::DatabaseError(err)
    }
}

impl From<mongodb::bson::de::Error> for ErrorKind {
    fn from(err: mongodb::bson::de::Error) -> Self {
        ErrorKind::DataIntergrityError(err)
    }
}

impl From<mongodb::bson::ser::Error> for ErrorKind {
    fn from(err: mongodb::bson::ser::Error) -> Self {
        ErrorKind::DataParseError(err)
    }
}

impl From<mongodb::bson::oid::Error> for ErrorKind {
    fn from(err: mongodb::bson::oid::Error) -> Self {
        ErrorKind::BadInput(err)
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
    code: &'static str,
    kind: ErrorKind,
}

impl Error {
    pub fn new(msg: impl Into<String>, kind: impl Into<ErrorKind>) -> Error {
        Error {
            msg: msg.into(),
            code: "NA",
            kind: kind.into(),
        }
    }

    pub fn with_code(
        msg: impl Into<String>,
        code: &'static str,
        kind: impl Into<ErrorKind>,
    ) -> Error {
        Error {
            msg: msg.into(),
            code,
            kind: kind.into(),
        }
    }

    pub fn msg(&self) -> &String {
        &self.msg
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Internal => None,
            ErrorKind::InvalidData => None,
            ErrorKind::NotFound => None,
            ErrorKind::UnAuthorized => None,
            ErrorKind::UnInitializedConnectionManager => None,
            ErrorKind::DatabaseError(err) => Some(err),
            ErrorKind::BadInput(err) => Some(err),
            ErrorKind::DataIntergrityError(err) => Some(err),
            ErrorKind::DataParseError(err) => Some(err),
            ErrorKind::LogicalError => None,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    msg: String,
    code: String,
}

impl ErrorResponse {
    pub fn new(code: impl Into<String>, msg: impl Into<String>) -> ErrorResponse {
        ErrorResponse {
            code: code.into(),
            msg: msg.into(),
        }
    }
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.kind {
            ErrorKind::Internal => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::InvalidData => actix_web::http::StatusCode::BAD_REQUEST,
            ErrorKind::UnAuthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            ErrorKind::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            ErrorKind::UnInitializedConnectionManager => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            ErrorKind::DataParseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::BadInput(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ErrorKind::DatabaseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::DataIntergrityError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::LogicalError => actix_web::http::StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse::new(self.code.to_string(), self.msg.to_string());
        actix_web::HttpResponse::build(status_code).json(error_response)
    }
}

pub trait Context<T> {
    fn context(self, msg: impl Into<String>) -> Result<T>;

    fn handover(self) -> Result<T>;

    fn with_context<F, S>(self, cb: F) -> Result<T>
    where
        F: Fn() -> S,
        S: Into<String>;
}

impl<T, E: Into<ErrorKind> + fmt::Display> Context<T> for result::Result<T, E> {
    fn context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|err| Error::new(msg, err))
    }

    fn handover(self) -> Result<T> {
        self.map_err(|err| Error::new(err.to_string(), err))
    }

    fn with_context<F, S>(self, cb: F) -> Result<T>
    where
        F: Fn() -> S,
        S: Into<String>,
    {
        self.map_err(move |err| Error::new(cb(), err))
    }
}
