use core::fmt;
#[allow(dead_code)]

// SUMMARY
// enum ≠ string
// enum = safe value
// Display = the way to write value
// one value → many representation
// Rust divie data & representation

// important points usage Debug, Display or manual mapping
// Log developer            ->  Debug
// Output CLI               ->  Display
// HTTP response message	->  Display
// JSON API	                ->  serde
// Database	                ->  manual mapping

// enum is like data type, let status = Status::Active; this means value with type Status
// this because one enum have many representation, ex: on CLI, log, JSON, DB
// main point: enum is possibility contract NOT text/number/boolean representation.
enum Status {
    Active,
    Inactive,
}

// IMPL
// 1. impl must write on module level and file level.
// 2. impl define type not statement/expression.
// 3. not allowed write inside function, loop, if, and test function.

// DISPLAY
// 1. because enum have many representation, th Display answer it. display is used to display the enum based on the context.
// 2. for example in CLI want to display like this, and then for user display like that, on log etc.
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => write!(f, "active"),
            Status::Inactive => write!(f, "inactive"),
        }
    }
}

enum LampStatus {
    On,
    Off,
}

impl fmt::Display for LampStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LampStatus::On => write!(f, "lamp is on"),
            LampStatus::Off => write!(f, "lamp is off"),
        }
    }
}

impl LampStatus {
    fn is_on(&self) -> bool {
        match self {
            LampStatus::On => true,
            LampStatus::Off => false,
        }
    }
}

enum LoginResult {
    Success(String),
    Failed,
}

impl LoginResult {
    fn message(&self) -> String {
        match self {
            LoginResult::Success(name) => format!("welcome {}", name).to_string(),
            LoginResult::Failed => "login failed".to_string(),
        }
    }
}

enum OrderStatus {
    Created,
    Paid,
    Canceled,
}

impl OrderStatus {
    fn can_ship(&self) -> bool {
        match self {
            OrderStatus::Created => false,
            OrderStatus::Paid => true,
            OrderStatus::Canceled => false,
        }
    }
}

enum UserStatus {
    Active,
    Banned,
}

impl UserStatus {
    fn login(&self) -> Result<(), String> {
        match self {
            UserStatus::Active => Ok(()),
            UserStatus::Banned => Err(String::from("user banned")),
        }
    }
}


#[cfg(test)]
mod test_enum {
    use crate::_enum::{LampStatus, LoginResult, OrderStatus, Status, UserStatus};

    #[test]
    fn test_enum_status() {
        let status_active = Status::Active;
        println!("{}", status_active);

        let status_inactive = Status::Inactive;
        println!("{}", status_inactive);
    }

    #[test]
    fn test_enum_two() {
        let mut lamp: LampStatus = LampStatus::On;
        assert_eq!(lamp.is_on(), true);

        lamp = LampStatus::Off;
        assert_eq!(lamp.is_on(), false);

        println!("{}", lamp)
    }

    #[test]
    fn test_enum_login() {
        let login_success = LoginResult::Success("fajar".to_string());
        assert_eq!(login_success.message(), "welcome fajar");

        let login_failed: LoginResult = LoginResult::Failed;
        assert_eq!(login_failed.message(), "login failed")
    }

    #[test]
    fn test_order_status() {
        let mut order_status = OrderStatus::Paid;
        assert_eq!(order_status.can_ship(), true);

        order_status = OrderStatus::Created;
        assert_eq!(order_status.can_ship(), false);

        order_status = OrderStatus::Canceled;
        assert_eq!(order_status.can_ship(), false);
    }

    #[test]
    fn test_user_status() {
        let user_status_active = UserStatus::Active;
        assert_eq!(user_status_active.login(), Ok(()));

        let user_status_banned = UserStatus::Banned;
        assert_eq!(user_status_banned.login(), Err("user banned".to_string()))
    }
}
