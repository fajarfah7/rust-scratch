use core::fmt;

fn add_string(fname: &str, lname: &str) -> String {
    format!("{} {}", fname, lname)
}

// fname here is mutable borrow(owner still same, but can change value)
fn append_string(fname: &mut String, lname: &str) {
    fname.push_str(" ");
    fname.push_str(lname);
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn process() -> Result<(), String> {
    let a = divide(10, 2);
    match a {
        Ok(v) => println!("{}", v),
        Err(e) => return Err(e),
    }
    Ok(())
}

// shortened without match can use "?", and then MUST followd by Ok(...)
fn shortened_process() -> Result<(), String> {
    let v = divide(10, 0)?;
    println!("{}", v);
    Ok(())
}


fn step1() -> Result<i32, String> {
    Ok(10)
}
fn step2(v: i32) -> Result<i32, String> {
    Ok(v * 10)
}
fn chaining_process() -> Result<i32, String> {
    let v = step1()?;
    let v = step2(v)?;
    Ok(v)
}

// Debug => log internal
#[derive(Debug)]
pub enum ApiError {
    Validation(String),
    NotFound,
    Unauthorized,
    Internal,
}
// Display => message to user
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Validation(msg) => write!(f, "validation error: {}", msg),
            ApiError::NotFound => write!(f, "resource not found"),
            ApiError::Unauthorized => write!(f, "unauthorized"),
            ApiError::Internal => write!(f, "internal server error"),
        }
    }
}
impl ApiError {
    pub fn status_code(&self) -> u16 {
        match self {
            ApiError::Validation(_) => 400,
            ApiError::Unauthorized => 403,
            ApiError::NotFound => 404,
            ApiError::Internal => 500,
        }
    }
}

fn parse_age(input: &str) -> Result<u32, ApiError> {
    let age: u32 = input.parse().map_err(|_| ApiError::Validation("age must be a number".into()))?; // |_| closure anonymous function |parameter|
    if age < 18 {
        return Err(ApiError::Validation("age must be >= 18".into()));
    }
    Ok(age)
}
fn handler_register_user(input: &str) -> Result<String, ApiError> {
    let age = parse_age(input)?;
    Ok(format!("user age: {}", age))
}

async fn hello() -> &'static str {
    "hello"
}

#[cfg(test)]
mod test_function {
    use super::*;

    #[test]
    fn test_add_string() {
        let fname: String = String::from("Fajar");
        let lname: &str = "Fahrurozi";
        let full_name = add_string(&fname, lname);
        assert_eq!(full_name, format!("{} {}", fname, lname))
    }

    #[test]
    fn test_append_string() {
        let mut fname: String = String::from("Fajar");
        let lname: &str = "Fahrurozi";
        append_string(&mut fname, lname);
        assert_eq!(fname, "Fajar Fahrurozi".to_string())
    }

    #[test]
    fn test_divide() {
        let a = divide(10, 2);
        match a {
            Ok(val) => println!("val: {}", val),
            Err(err) => println!("err: {}", err),
        }
    }

    #[test]
    fn test_process() {
        let a = process();
        match a {
            Ok(()) => println!("ok"),
            Err(err) => println!("err: {}", err),
        }
    }

    #[test]
    fn test_shortened_process() {
        let a = shortened_process();
        match a {
            Ok(()) => println!("ok"),
            Err(err) => println!("err: {}", err),
        }
    }

    #[test]
    fn test_chaining_process() {
        let a = chaining_process();
        match a {
            Ok(v) => println!("{}", v),
            Err(err) => println!("{}", err)
        }
    }

    #[test]
    fn test_handle_error() {
        let age: &str = "12";
        let register_result = handler_register_user(age);
        match register_result {
            Ok(res) => println!("{}", res),
            Err(err) => println!("{:?}", err),
        }
    }

    #[tokio::test]
    async fn test_hello_async() {
        let res = hello().await;
        assert_eq!(res, "hello");
    }
}
