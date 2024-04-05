use axum::http::header::WWW_AUTHENTICATE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sqlx::error::DatabaseError;
use std::borrow::Cow;
use std::collections::HashMap;

/// A common error type for user throughout the API.
/// Can be returned in a Result monad from a handler.
/// Represents both API eroors and internal server errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// 401 Unauthorized
    #[error("authentication required")]
    Unauthorized,

    /// 403 Forbidden
    #[error("user may not perform that action")]
    Forbidden,

    /// 404 Not Found
    #[error("resource not found")]
    NotFound,

    /// 422 Unprocessable Entity
    #[error("invalid request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    /// 500 Internal Server Error for Sqlx errors
    #[error("internal database error")]
    Sqlx(#[from] sqlx::Error),

    /// 500 Internal Server Error for anyhow errors
    #[error("internal server error")]
    Anyhow(#[from] anywhow::Error),
}

impl Error {
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::UnprocessableEntity { errors } => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
                }

                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response();
            }
            Self::Unauthorized => {
                return (
                    self.status_code(),
                    // Include the `WWW-Authenticate` challenge required in the specification
                    [(WWW_AUTHENTICATE, "Token")],
                    self.to_string(),
                )
                    .into_response();
            }

            Self::Sqlx(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP request by `TraceLayer`.
                log::error!("SQLx error: {:?}", e);
            }

            Self::Anyhow(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP request by `TraceLayer`.
                log::error!("Generic error: {:?}", e);
            }

            // Other errors get mapped normally.
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}

/// A little helper trait for more easily converting database constraint errors into API errors.
///
/// ```rust,ignore
/// let user_id = sqlx::query_scalar!(
///     r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
///     username,
///     email,
///     password_hash
/// )
///     .fetch_one(&ctxt.db)
///     .await
///     .on_constraint("user_username_key", |_| Error::unprocessable_entity([("username", "already taken")]))?;
/// ```
///
/// Something like this would ideally live in a `sqlx-axum` crate if it made sense to author one,
/// however its definition is tied pretty intimately to the `Error` type, which is itself
/// tied directly to application semantics.
///
/// To actually make this work in a generic context would make it quite a bit more complex,
/// as you'd need an intermediate error type to represent either a mapped or an unmapped error,
/// and even then it's not clear how to handle `?` in the unmapped case without more boilerplate.
pub trait ResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    ///
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}
