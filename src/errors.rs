use axum::http::StatusCode;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
    {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }

pub fn not_found_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
    {
        (StatusCode::NOT_FOUND, err.to_string())
    }

pub fn found_error<E>(err: E) -> (StatusCode, String) 
    where
        E: std::error::Error,
    {
        (StatusCode::EXPECTATION_FAILED, err.to_string())
    }