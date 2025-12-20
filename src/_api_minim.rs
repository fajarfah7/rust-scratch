use core::fmt;

use serde::{Serialize, Deserialize};

enum ResponseError {
    BadRequest(String)
}
impl fmt::Debug for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::BadRequest(msg) => write!(f, "bad_request: {}", msg),
        }
    }
}
impl fmt::Display for ResponseError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::BadRequest(msg) => write!(f, "bad_request: {}", msg),
        }
    }
}
impl From<axum::extract::rejection::JsonRejection> for ResponseError {
    fn from(err: axum::extract::rejection::JsonRejection) -> Self {
        ResponseError::BadRequest(err.to_string())
    }
}
impl axum::response::IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ResponseError::BadRequest(msg) => {(axum::http::StatusCode::BAD_REQUEST, msg).into_response()}
        }
    }
}

async fn get_by_id(axum::extract::Path(id): axum::extract::Path<u32>) -> String {
    format!("get user with id: {}", id)
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: Option<String>,
    name: String,
    email: String,
}
async fn update_user(axum::extract::Path(id): axum::extract::Path<String>,axum::Json(payload): axum::Json<User>) -> Result<axum::Json<User>, ResponseError> {
    if payload.email == "" {
        return Err(ResponseError::BadRequest("invalid email".into()))
    }
    Ok(axum::Json(User { id: Some(id), name: payload.name, email: payload.email }))
}

#[cfg(test)]
mod test_api_minim {
    use std::usize;
    use tower::ServiceExt;
    use crate::_api_minim::{get_by_id, update_user};

    #[tokio::test]
    async fn test_router() {
        let app: axum::Router = axum::Router::new().route("/", axum::routing::get(|| async {"ok"}));

        let response = app.oneshot(
            axum::http::Request::builder()
                .uri("/")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(response.status(), 200)
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let app: axum::Router = axum::Router::new().route("/user/:id", axum::routing::get(get_by_id));

        let response = app.oneshot(
            axum::http::Request::builder()
            .uri("/user/3")
            .body(axum::body::Body::empty())
            .unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(response.status(), 200);

        let response_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
        // println!("response_body: {}",response_body);
        assert_eq!(response_body, "get user with id: 3");
    }

    #[tokio::test]
    async fn test_update_user() {
        let request_json = r#"{"name": "Fajar","email":"fajar@gmail.com"}"#;
        let app: axum::Router = axum::Router::new().route("/user/:id", axum::routing::put(update_user));

        let response = app.oneshot(
            axum::http::Request::builder()
            .method("PUT")
            .uri("/user/3")
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(request_json))
            .unwrap()
        )
        .await
        .unwrap();
        
        // assert_eq!(response.status(), 200);

        let response_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

        println!("response_body: {:?}", response_body)
    }
}