use core::fmt;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum ApiError {
    InvalidJson(String),
    AuthError(String),
}
impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::InvalidJson(msg) => {
                write!(f, "ApiError::InvalidJson(\"{}\")", msg)
            }
            ApiError::AuthError(msg) => {
                write!(f, "ApiError::AuthError(\"{}\")", msg)
            }
        }
    }
}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::InvalidJson(msg) => {
                write!(f, "invalid body request: {}", msg)
            }
            ApiError::AuthError(_) => {
                write!(f, "wrong username or password")
            }
        }
    }
}
impl From<serde_json::Error> for ApiError{
    fn from(msg: serde_json::Error) -> Self {
        ApiError::InvalidJson(msg.to_string())
    }
}
impl ApiError {
    fn status_code(&self) -> u16 {
        match self {
            ApiError::InvalidJson(_) | ApiError::AuthError(_) => 400,
        }
    }
}

#[derive(Deserialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}
fn decode_login_request(s: &str) -> Result<LoginRequest, ApiError> {
    let v = serde_json::from_str(s)?;
    Ok(v)
}
fn login(json_request: &str) -> Result<LoginResponse, ApiError> {
    let request = decode_login_request(json_request)?;
    // let username = request.username;
    // let password = request.password;

    // DESTRUCTING STYLE
    let LoginRequest {username, password} = request;
    if username != "admin" || password != "password" {
        return Err(ApiError::AuthError("wrong username or password".into()))
    }
    let login_response = LoginResponse {
        id: "ADM-001".into(),
        name: "Admin".into(),
        is_active: true,
    };
    Ok(login_response)
}

#[derive(Serialize, Debug)]
struct LoginResponse {
    id: String,
    name: String,
    is_active: bool,
}
impl LoginResponse {
    fn encode(&self) -> Result<String, ApiError> {
        let s = serde_json::to_string(self)?;
        Ok(s)
    }
}

#[cfg(test)]
mod test_json {
    use crate::_json::{Point, login};

    #[test]
    fn test_json_point() {
        let point = Point{x: 1, y: 2};
        let serialized = serde_json::to_string(&point).unwrap();
        println!("serialized = {}", serialized);
        
        let deserialized: Point = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
    }

    #[test]
    fn test_json_login() {
        let login_request =  r#"{ "username": "admin", "password": "passwordd" }"#;
        let login_result = login(login_request);
        match login_result {
            Ok(response) => {
                let json_message = response.encode();
                match json_message {
                    Ok(msg) => println!("data:{}", msg),
                    Err(msg) => println!("error:{}", msg)
                }
            }
            Err(e) => {
                println!("DEBUG:{:?}", e);
                println!("DISPLAY: {}", e);
                println!("STATUS: {}", e.status_code());
            }
        }
    }
}