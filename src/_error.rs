use core::fmt;
#[warn(dead_code)]

// RECOMENDED STRONGLY, every enum MUST have DEBUG & DISPLAY
// DEBUG: is used for developer for logging/debuging purpose.
// DISPLAY: is used for ux / user.
enum MyCustomErr {
    CustomErrOne(String),
    CustomErrTwo(String),
}
impl fmt::Debug for MyCustomErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyCustomErr::CustomErrOne(msg) => {
                write!(f, "MyCustomErr::CustomErrOne(msg=\"{}\")", msg)
            }
            MyCustomErr::CustomErrTwo(msg) => {
                write!(f, "MyCustomErr::CustomErrTwo(msg=\"{}\")", msg)
            }
        }
    }
}
impl fmt::Display for MyCustomErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyCustomErr::CustomErrOne(s) => write!(f, "Custom error one: {}", s),
            MyCustomErr::CustomErrTwo(s) => write!(f, "Custom error two: {}", s),
        }
    }
}
impl MyCustomErr  {
    fn get_status_error(&self) -> u16 {
        match self {
            MyCustomErr::CustomErrOne(_) => 401,
            MyCustomErr::CustomErrTwo(_) => 402,
        }
    }
}
fn login(uname: &str, pswd: &str) -> Result<String, MyCustomErr> {
    if uname != "admin" {
        return Err(MyCustomErr::CustomErrOne("wrong username".into()))
    }
    if pswd != "password" {
        return Err(MyCustomErr::CustomErrTwo("wrong password".into()))
    }
    Ok(format!("welcome admin"))
}
fn handle_error<E>(err: E) where E: std::fmt::Debug + std::fmt::Display {
    eprintln!("DEBUG    : {:?}", err);
    eprintln!("DISPLAY  : {}", err);
}

// // Another example to create Debug with some variant.
// enum ApiError {
//     NotFound,
//     Unauthorized { reason: String },
//     Validation(String),
// }
// impl fmt::Debug for ApiError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ApiError::NotFound => write!(f, "ApiError::NotFound"),
//             ApiError::Validation(msg) => {
//                 write!(f, "ApiError::Validation(msg=\"{}\")", msg)
//             }
//             ApiError::Unauthorized { reason } => {
//                 write!(f, "ApiError::Unauthorized(reason=\"{}\")", reason)
//             }
//         }
//     }
// }

enum AppError {
    InvalidValue(std::num::ParseIntError),
    InvalidInput(String),
}
impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValue(msg) => {
                write!(f, "AppError::InvalidValue(\"{}\")", msg)
            }
            Self::InvalidInput(msg) => {
                write!(f, "AppError::InvalidInput(\"{}\")", msg)
            }
        }
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidValue(msg) => write!(f, "invalid value: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "invalid input: {}", msg)
        }
    }
}
// FROM: if you implement this error type can use this enum type for several purpose
// 1. handle invalid value in parsing str -> num process
//      in this process Rust will check what there is impl From<ParseIntError> for AppError
//      if exist then use, else compile error
// 2. handle invalid input to handle login process
// all of them use same error type
impl From<std::num::ParseIntError> for AppError{
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::InvalidValue(err)
    }
}
fn to_number(s: &str) -> Result<u32, AppError> {
    // let v: u32 = s.parse::<u32>().map_err(|e| {
    //     AppError::InvalidValue(e)
    // })?;
    let v = s.parse::<u32>()?;
    if v < 18 {
        return Err(AppError::InvalidInput("age is under 18".into()))
    }
    Ok(v)
}
fn login_to_app(uname: &str, pswd: &str) -> Result<String, AppError> {
    if uname != "admin" {
        return Err(AppError::InvalidInput("wrong username".into()))
    }
    if pswd != "password" {
        return Err(AppError::InvalidInput("wrong password".into()))
    }
    Ok(format!("welcome admin"))
}
#[cfg(test)]
mod test_error {
    use crate::_error::{handle_error, login, login_to_app, to_number};

    #[test]
    fn test_to_number() {
        let v: &str = "-12";
        let n = to_number(v);
        match n {
            Ok(val) => println!("{}", val),
            Err(err) => {
                println!("DEBUG: {:?}", err);
                println!("DISPLAY: {}", err)
            } // {} display, {:?} debug
        }
    }
    #[test]
    fn test_login_into_app() {
        let login = login_to_app("admin", "passwordd");
        match login {
            Ok(msg) => println!("{}", msg),
            Err(err) => {
                println!("DEBUG: {:?}", err);
                println!("DISPLAY: {}", err)
            }
        }
    }

    #[test]
    fn test_login() {
        let uname = "adminn";
        let pswd = "password";
        let result = login(uname, pswd);
        match result {
            Ok(msg) => println!("{}", msg),
            Err(msg) => {
                println!("DISPLAY   :{}", msg);
                println!("DEBUG     :{:?}", msg);
                println!("STATUS    :{}", msg.get_status_error());
            },
        }
        if let Err(e) = login(uname, pswd) {
            handle_error(e);
        }
    }
}